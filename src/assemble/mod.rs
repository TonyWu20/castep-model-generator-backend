/// Assemble adsorbate and lattice.
use std::{collections::HashMap, error::Error};

use na::{Point3, Translation3, Unit, UnitQuaternion, Vector3};

use crate::{
    adsorbate::Adsorbate, lattice::LatticeModel, math_helper::centroid_of_points,
    model_type::ModelInfo, Transformation,
};

pub trait AdsorptionState {}

#[derive(Default, Debug)]
pub struct Imported {}

#[derive(Default, Debug)]
pub struct Aligned {}

#[derive(Default, Debug)]
pub struct PlaneAdjusted {}

#[derive(Default, Debug)]
pub struct Ready {}

impl AdsorptionState for Imported {}
impl AdsorptionState for Aligned {}
impl AdsorptionState for PlaneAdjusted {}
impl AdsorptionState for Ready {}

#[derive(Debug)]
pub struct AdsorptionBuilder<T: ModelInfo + Clone, U: AdsorptionState> {
    host_lattice: LatticeModel<T>,
    adsorbate: Option<LatticeModel<T>>,
    location: Option<Point3<f64>>,
    ads_direction: Option<Vector3<f64>>,
    coord_angle: Option<f64>,
    adsorbate_plane_angle: Option<f64>,
    height: Option<f64>,
    state: U,
}

impl<T: ModelInfo + Clone> AdsorptionBuilder<T, Imported> {
    pub fn new(host_lattice: LatticeModel<T>) -> Self {
        Self {
            host_lattice,
            adsorbate: None,
            location: None,
            coord_angle: None,
            adsorbate_plane_angle: None,
            height: None,
            ads_direction: None,
            state: Imported {},
        }
    }
    pub fn add_adsorbate(mut self, adsorbate_lattice: LatticeModel<T>) -> Self {
        self.adsorbate = Some(adsorbate_lattice);
        self
    }
    pub fn set_location(mut self, target_coord_sites: &[u32]) -> Self {
        let coord_points: Vec<&Point3<f64>> = target_coord_sites
            .iter()
            .map(|&site_id| self.host_lattice.get_atom_by_id(site_id).unwrap().xyz())
            .collect();
        let centroid = centroid_of_points(&coord_points);
        self.location = Some(centroid);
        self
    }
    pub fn set_ads_direction(mut self, direction_vec: &Vector3<f64>) -> Self {
        self.ads_direction = Some(direction_vec.to_owned());
        self
    }
    /**
    Determines the angle between the adsorbate and target sites.
    If the adsorbate is singly-coordinated, this determines the angle of the adsorbate stem to the target site atom.
    If the adsorbate has more than one coordinated atom, the angle should be 0.0 to set to be parallel.
    */
    pub fn set_coord_angle(mut self, angle: f64) -> Self {
        self.coord_angle = Some(angle);
        self
    }
    pub fn set_height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }
    /// Determines the adsorbate plane angel relative to the host target sites.
    /// Example: When set to 0.0, it means the adsorbate is placed "flat". When set to 90.0, it means
    /// it means the adsorbate plane is perpendicular to the host.
    pub fn set_adsorbate_plane_angle(mut self, plane_angle: f64) -> Self {
        self.adsorbate_plane_angle = Some(plane_angle);
        self
    }
    pub fn align_ads(mut self, stem_atom_ids: &[u32]) -> AdsorptionBuilder<T, Aligned> {
        let stem_vector = self
            .adsorbate
            .as_ref()
            .unwrap()
            .get_vector_ab(stem_atom_ids[0], stem_atom_ids[1])
            .unwrap();
        let direction_vec = self.ads_direction.unwrap();
        let angle_stem_direction = stem_vector.angle(&direction_vec);
        let rot_axis = Unit::new_normalize(stem_vector.cross(&direction_vec));
        let rot_quatd = UnitQuaternion::from_axis_angle(&rot_axis, angle_stem_direction);
        self.adsorbate.as_mut().unwrap().rotate(&rot_quatd);
        AdsorptionBuilder {
            host_lattice: self.host_lattice,
            adsorbate: self.adsorbate,
            location: self.location,
            ads_direction: self.ads_direction,
            coord_angle: self.coord_angle,
            adsorbate_plane_angle: self.adsorbate_plane_angle,
            height: self.height,
            state: Aligned {},
        }
    }
}

impl<T: ModelInfo + Clone> AdsorptionBuilder<T, Aligned> {
    /// Rotate the adsorbate to the desired plane direction, before adsorption
    /// This step adjust the adsorbate stem direction ranging from parallel to the host plane to vertical
    /// to the host plane. Transit to `PlaneAdjusted` state.
    pub fn init_ads_plane_direction(
        mut self,
        plane_atom_ids: &[u32],
    ) -> AdsorptionBuilder<T, PlaneAdjusted> {
        let target_plane_angle = self.adsorbate_plane_angle.unwrap();
        let ads = self.adsorbate.as_ref().unwrap();
        let plane_ba = ads
            .get_vector_ab(plane_atom_ids[0], plane_atom_ids[1])
            .unwrap();
        let plane_ca = ads
            .get_vector_ab(plane_atom_ids[0], plane_atom_ids[2])
            .unwrap();
        let plane_normal = plane_ba.cross(&plane_ca);
        let z_axis = Vector3::z_axis();
        // Let the rotate direction is from normal to z_axis.
        let rot_axis = Unit::new_normalize(plane_normal.cross(&z_axis));
        let normal_to_z_angle = plane_normal.angle(&z_axis);
        // The angle needed to rotate is the difference between current plane normal to z-axis angle and the desired angle.
        let rot_angle = normal_to_z_angle - target_plane_angle;
        let rot_quatd = UnitQuaternion::from_axis_angle(&rot_axis, rot_angle);
        self.adsorbate.as_mut().unwrap().rotate(&rot_quatd);
        AdsorptionBuilder {
            host_lattice: self.host_lattice,
            adsorbate: self.adsorbate,
            location: self.location,
            ads_direction: self.ads_direction,
            coord_angle: self.coord_angle,
            adsorbate_plane_angle: self.adsorbate_plane_angle,
            height: self.height,
            state: PlaneAdjusted {},
        }
    }
}

impl<T: ModelInfo + Clone> AdsorptionBuilder<T, PlaneAdjusted> {
    /**
    When the adsorbate has single coordination atom, translate the adsorbate
    to a position that the distance between coord site and target site is the
    input bond distance, while the bond direction follows the adsorbate direction.
    */
    fn single_coord(&mut self, stem_atom_ids: &[u32], coord_atom_id: u32, bond_distance: f64) {
        let ads = self.adsorbate.as_ref().unwrap();
        let location = self.location.unwrap();
        let coord_atom_point = ads.get_atom_by_id(coord_atom_id).unwrap().xyz();
        // Create a stem_vector pointing upwards
        let stem_vector = {
            let stem_atom_1 = ads.get_atom_by_id(stem_atom_ids[0]).unwrap();
            let stem_atom_2 = ads.get_atom_by_id(stem_atom_ids[1]).unwrap();
            if stem_atom_2.xyz().z > stem_atom_1.xyz().z {
                stem_atom_2.xyz() - stem_atom_1.xyz()
            } else {
                stem_atom_1.xyz() - stem_atom_2.xyz()
            }
        };
        /*
        The stem pointing out from the target location. This is to find the
        direction of the vector from coordination atom to target site.
        */
        let stem_from_loc = Vector3::new(
            location.x + stem_vector.x,
            location.y + stem_vector.y,
            location.z + stem_vector.z,
        );
        // Scaled the above vector to the bonding distance
        let scaled_stem_from_loc = stem_from_loc * (bond_distance / stem_from_loc.norm());
        // Compute the actual position for the coordination atom to reach.
        let actual_position = Point3::new(
            location.x + scaled_stem_from_loc.x,
            location.y + scaled_stem_from_loc.y,
            location.z + scaled_stem_from_loc.z,
        );
        let translate_mat = Translation3::from(actual_position - coord_atom_point);
        self.adsorbate.as_mut().unwrap().translate(&translate_mat);
    }
    /**
    When the adsorbate has multiple coordination atoms, translate
    the adsorbate from centroid of coord atoms to the location (centroid of target sites)
    */
    fn multiple_coord(&mut self, coord_atom_ids: &[u32], bond_distance: f64) {
        let ads = self.adsorbate.as_ref().unwrap();
        let mut location = self.location.unwrap();
        if coord_atom_ids.len() > 1 {
            let coord_atom_points: Vec<&Point3<f64>> = coord_atom_ids
                .iter()
                .map(|&coord_id| ads.get_atom_by_id(coord_id).unwrap().xyz())
                .collect();
            let coord_centroid = centroid_of_points(&coord_atom_points);
            location.z += bond_distance;
            let translate_mat = Translation3::from(location - coord_centroid);
            self.adsorbate.as_mut().unwrap().translate(&translate_mat);
        }
    }
    /// Place the adsorbate, depending on the number of coordination atoms.
    /// Transit to `Ready` state.
    pub fn place_adsorbate(
        mut self,
        stem_atom_ids: &[u32],
        coord_atom_ids: &[u32],
        bond_distance: f64,
    ) -> AdsorptionBuilder<T, Ready> {
        /*
        When the adsorbate has multiple coordination atoms, translate
        the adsorbate from centroid of coord atoms to the location (centroid of target sites)
        */
        if coord_atom_ids.len() > 1 {
            self.multiple_coord(coord_atom_ids, bond_distance);
        } else {
            self.single_coord(stem_atom_ids, coord_atom_ids[0], bond_distance);
        }
        AdsorptionBuilder {
            host_lattice: self.host_lattice,
            adsorbate: self.adsorbate,
            location: self.location,
            ads_direction: self.ads_direction,
            coord_angle: self.coord_angle,
            adsorbate_plane_angle: self.adsorbate_plane_angle,
            height: self.height,
            state: Ready {},
        }
    }
}

impl<T> AdsorptionBuilder<T, Ready>
where
    T: ModelInfo + Clone,
{
    pub fn build_adsorbed_lattice(mut self) -> LatticeModel<T> {
        let last_atom_id = self.host_lattice.atoms().len() as u32;
        self.adsorbate
            .as_mut()
            .unwrap()
            .atoms_mut()
            .into_iter()
            .for_each(|atom| {
                atom.set_atom_id(atom.atom_id() + last_atom_id);
            });
        let ads_atoms = self.adsorbate.as_ref().unwrap().atoms().to_vec();
        for atom in ads_atoms.into_iter() {
            self.host_lattice.atoms_mut().push(atom)
        }
        let all_atoms = self.host_lattice.atoms().to_vec();
        let lattice_vectors = self.host_lattice.lattice_vectors().unwrap().clone();
        LatticeModel::new(
            Some(lattice_vectors),
            all_atoms,
            self.host_lattice.model_type().to_owned(),
        )
    }
}
/// For lattice that can add adsorbate. The adsorbate must implement `AdsorbateTraits` and `Clone`
pub trait AddAdsorbate {
    /// Generate suffix about adsorbate and coordination sites.
    fn append_mol_name<T: Adsorbate + Clone>(
        &mut self,
        ads: &T,
        target_sites: &[u32],
        coord_site_dict: &HashMap<u32, String>,
    ) -> Result<(), Box<dyn Error>>;
}
