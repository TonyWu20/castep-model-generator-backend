/// Assemble adsorbate and lattice.
use std::{collections::HashMap, error::Error, f64::consts::PI, fmt::Debug, marker::PhantomData};

use na::{Point3, Translation3, Unit, UnitQuaternion, Vector3};

use crate::{
    adsorbate::Adsorbate,
    builder_typestate::{No, ToAssign, Yes},
    lattice::LatticeModel,
    math_helper::centroid_of_points,
    model_type::ModelInfo,
    Transformation,
};

pub trait BuilderState {}
pub trait ParamSetState {}

#[derive(Default, Debug)]
pub struct BareLattice;

#[derive(Default, Debug)]
pub struct Imported;

#[derive(Default, Debug)]
pub struct ParamSet;

#[derive(Default, Debug)]
pub struct Aligned;

#[derive(Default, Debug)]
pub struct PlaneAdjusted;

#[derive(Default, Debug)]
pub struct Ready;

impl BuilderState for BareLattice {}
impl BuilderState for Imported {}
impl BuilderState for ParamSet {}
impl BuilderState for Aligned {}
impl BuilderState for PlaneAdjusted {}
impl BuilderState for Ready {}

impl ParamSetState for ParamSet {}
impl ParamSetState for Aligned {}
impl ParamSetState for PlaneAdjusted {}
impl ParamSetState for Ready {}

#[derive(Debug)]
pub struct AdsorptionBuilder<T: ModelInfo + Clone, U: BuilderState> {
    host_lattice: LatticeModel<T>,
    adsorbate: Option<LatticeModel<T>>,
    location: Option<Point3<f64>>,
    ads_params: Option<AdsParamsBuilder<Yes, Yes, Yes>>,
    state: PhantomData<U>,
}

#[derive(Debug, Default)]
pub struct AdsParamsBuilder<AdsDirSet, PlaneAngleSet, BondLengthSet>
where
    AdsDirSet: ToAssign,
    PlaneAngleSet: ToAssign,
    BondLengthSet: ToAssign,
{
    ads_direction: Option<Vector3<f64>>,
    adsorbate_plane_angle: Option<f64>,
    bond_length: Option<f64>,
    ads_direction_set: PhantomData<AdsDirSet>,
    adsorbate_plane_angle_set: PhantomData<PlaneAngleSet>,
    bond_length_set: PhantomData<BondLengthSet>,
}

impl<AdsDirSet, PlaneAngleSet, BondLengthSet>
    AdsParamsBuilder<AdsDirSet, PlaneAngleSet, BondLengthSet>
where
    AdsDirSet: ToAssign,
    PlaneAngleSet: ToAssign,
    BondLengthSet: ToAssign,
{
    pub fn new() -> AdsParamsBuilder<No, No, No> {
        AdsParamsBuilder {
            ads_direction: None,
            adsorbate_plane_angle: None,
            bond_length: None,
            ads_direction_set: PhantomData,
            adsorbate_plane_angle_set: PhantomData,
            bond_length_set: PhantomData,
        }
    }
    pub fn with_ads_direction(
        mut self,
        ads_direction: &Vector3<f64>,
    ) -> AdsParamsBuilder<Yes, PlaneAngleSet, BondLengthSet> {
        self.ads_direction = Some(ads_direction.to_owned());
        let Self {
            ads_direction,
            adsorbate_plane_angle,
            bond_length,
            ads_direction_set: _,
            adsorbate_plane_angle_set,
            bond_length_set,
        } = self;
        AdsParamsBuilder {
            ads_direction,
            adsorbate_plane_angle,
            bond_length,
            ads_direction_set: PhantomData,
            adsorbate_plane_angle_set,
            bond_length_set,
        }
    }
    // pub fn with_coord_angle(
    //     mut self,
    //     coord_angle: f64,
    // ) -> AdsParamsBuilder<AdsDirSet, Yes, PlaneAngleSet, BondLengthSet> {
    //     self.coord_angle = Some(coord_angle);
    //     let Self {
    //         ads_direction,
    //         coord_angle,
    //         adsorbate_plane_angle,
    //         bond_length,
    //         ads_direction_set,
    //         coord_angle_set: _,
    //         adsorbate_plane_angle_set,
    //         bond_length_set,
    //     } = self;
    //     AdsParamsBuilder {
    //         ads_direction,
    //         coord_angle,
    //         adsorbate_plane_angle,
    //         bond_length,
    //         ads_direction_set,
    //         coord_angle_set: PhantomData,
    //         adsorbate_plane_angle_set,
    //         bond_length_set,
    //     }
    // }
    pub fn with_plane_angle(
        mut self,
        plane_angle: f64,
    ) -> AdsParamsBuilder<AdsDirSet, Yes, BondLengthSet> {
        self.adsorbate_plane_angle = Some(plane_angle);
        let Self {
            ads_direction,
            adsorbate_plane_angle,
            bond_length,
            ads_direction_set,
            adsorbate_plane_angle_set: _,
            bond_length_set,
        } = self;
        AdsParamsBuilder {
            ads_direction,
            adsorbate_plane_angle,
            bond_length,
            ads_direction_set,
            adsorbate_plane_angle_set: PhantomData,
            bond_length_set,
        }
    }
    pub fn with_bond_length(
        mut self,
        bond_length: f64,
    ) -> AdsParamsBuilder<AdsDirSet, PlaneAngleSet, Yes> {
        self.bond_length = Some(bond_length);
        let Self {
            ads_direction,
            adsorbate_plane_angle,
            bond_length,
            ads_direction_set,
            adsorbate_plane_angle_set,
            bond_length_set: _,
        } = self;
        AdsParamsBuilder {
            ads_direction,
            adsorbate_plane_angle,
            bond_length,
            ads_direction_set,
            adsorbate_plane_angle_set,
            bond_length_set: PhantomData,
        }
    }
}

impl AdsParamsBuilder<Yes, Yes, Yes> {
    pub fn finish(self) -> Self {
        self
    }
}

impl<T, U> AdsorptionBuilder<T, U>
where
    T: ModelInfo + Clone,
    U: BuilderState + ParamSetState,
{
    fn ads_params(&self) -> &AdsParamsBuilder<Yes, Yes, Yes> {
        self.ads_params.as_ref().unwrap()
    }
    fn adsorbate_plane_angle(&self) -> f64 {
        self.ads_params().adsorbate_plane_angle.unwrap()
    }
    fn adsorbate(&self) -> &LatticeModel<T> {
        self.adsorbate.as_ref().unwrap()
    }
    fn location(&self) -> Point3<f64> {
        self.location.unwrap()
    }
    fn adsorbate_mut(&mut self) -> &mut LatticeModel<T> {
        self.adsorbate.as_mut().unwrap()
    }
    fn ads_direction(&self) -> &Vector3<f64> {
        self.ads_params().ads_direction.as_ref().unwrap()
    }
}

impl<T> AdsorptionBuilder<T, BareLattice>
where
    T: ModelInfo + Clone,
{
    pub fn new(host_lattice: LatticeModel<T>) -> Self {
        Self {
            host_lattice,
            adsorbate: None,
            location: None,
            ads_params: None,
            state: PhantomData,
        }
    }
    pub fn add_adsorbate(
        self,
        adsorbate_lattice: LatticeModel<T>,
    ) -> AdsorptionBuilder<T, Imported> {
        let Self {
            host_lattice,
            adsorbate: _,
            location,
            ads_params,
            state: _,
        } = self;
        AdsorptionBuilder {
            host_lattice,
            adsorbate: Some(adsorbate_lattice),
            location,
            ads_params,
            state: PhantomData,
        }
    }
}

impl<T: ModelInfo + Clone> AdsorptionBuilder<T, Imported> {
    pub fn with_location_at_sites(mut self, target_sites: &[u32]) -> Self {
        let target_sites_points: Vec<&Point3<f64>> = target_sites
            .iter()
            .map(|&site_id| self.host_lattice.get_atom_by_id(site_id).unwrap().xyz())
            .collect();
        let centroid = centroid_of_points(&target_sites_points);
        self.location = Some(centroid);
        self
    }
    pub fn with_ads_params(
        mut self,
        ads_params: AdsParamsBuilder<Yes, Yes, Yes>,
    ) -> AdsorptionBuilder<T, ParamSet> {
        self.ads_params = Some(ads_params);
        let Self {
            host_lattice,
            adsorbate,
            location,
            ads_params,
            state: _,
        } = self;
        AdsorptionBuilder {
            host_lattice,
            adsorbate,
            location,
            ads_params,
            state: PhantomData,
        }
    }
}

impl<T> AdsorptionBuilder<T, ParamSet>
where
    T: ModelInfo + Clone,
{
    /// Align the adsorbate stem with the given direction on host
    pub fn align_ads(mut self, stem_atom_ids: &[u32]) -> AdsorptionBuilder<T, Aligned> {
        let stem_vector = self
            .adsorbate()
            .get_vector_ab(stem_atom_ids[0], stem_atom_ids[1])
            .unwrap();
        let direction_vec = self.ads_direction();
        let angle_stem_direction = stem_vector.angle(direction_vec);
        let rot_axis = Unit::new_normalize(stem_vector.cross(direction_vec));
        let rot_quatd = UnitQuaternion::from_axis_angle(&rot_axis, angle_stem_direction);
        self.adsorbate_mut().rotate(&rot_quatd);
        let Self {
            host_lattice,
            adsorbate,
            location,
            ads_params,
            state: _,
        } = self;
        AdsorptionBuilder {
            host_lattice,
            adsorbate,
            location,
            ads_params,
            state: PhantomData,
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
        // Convert to radian
        let target_plane_angle = self.adsorbate_plane_angle() * PI / 180.0;
        let ads = self.adsorbate();
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
        // * The `angle` outputs in radian unit *
        let normal_to_z_angle = plane_normal.angle(&z_axis);
        // The angle needed to rotate is the difference between current plane normal to z-axis angle and the desired angle.
        let rot_angle = normal_to_z_angle - target_plane_angle;
        let rot_quatd = UnitQuaternion::from_axis_angle(&rot_axis, rot_angle);
        self.adsorbate_mut().rotate(&rot_quatd);
        let Self {
            host_lattice,
            adsorbate,
            location,
            ads_params,
            state: _,
        } = self;
        AdsorptionBuilder {
            host_lattice,
            adsorbate,
            location,
            ads_params,
            state: PhantomData,
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
        let ads = self.adsorbate();
        let location = self.location();
        let coord_atom_point = ads.get_atom_by_id(coord_atom_id).unwrap().xyz();
        // When the coord atom is on the stem
        if stem_atom_ids.contains(&coord_atom_id) {
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
            self.adsorbate_mut().translate(&translate_mat);
        }
        // Else, the coord atom is not on the stem, place it directly above the location with the `bond_distance`
        else {
            let actual_position = Point3::new(location.x, location.y, location.z + bond_distance);
            let translate_mat = Translation3::from(actual_position - coord_atom_point);
            self.adsorbate_mut().translate(&translate_mat);
        }
    }
    /**
    When the adsorbate has multiple coordination atoms, translate
    the adsorbate from centroid of coord atoms to the location (centroid of target sites)
    */
    fn multiple_coord(&mut self, coord_atom_ids: &[u32], bond_distance: f64) {
        let ads = self.adsorbate();
        let mut location = self.location();
        if coord_atom_ids.len() > 1 {
            let coord_atom_points: Vec<&Point3<f64>> = coord_atom_ids
                .iter()
                .map(|&coord_id| ads.get_atom_by_id(coord_id).unwrap().xyz())
                .collect();
            let coord_centroid = centroid_of_points(&coord_atom_points);
            location.z += bond_distance;
            let translate_mat = Translation3::from(location - coord_centroid);
            self.adsorbate_mut().translate(&translate_mat);
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
            ads_params: self.ads_params,
            state: PhantomData,
        }
    }
}

impl<T> AdsorptionBuilder<T, Ready>
where
    T: ModelInfo + Clone,
{
    pub fn build_adsorbed_lattice(self) -> LatticeModel<T> {
        self.host_lattice + self.adsorbate.unwrap()
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
