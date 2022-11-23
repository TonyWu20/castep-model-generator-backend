/// Assemble adsorbate and lattice.
use std::{f64::consts::PI, fmt::Debug, marker::PhantomData};

use na::{Point3, Translation3, Unit, UnitQuaternion, Vector3};

use crate::{
    builder_typestate::{No, ToAssign, Yes},
    error::InvalidCoord,
    lattice::LatticeModel,
    math_helper::{centroid_of_points, line_plane_intersect, plane_normal},
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
pub struct Calibrated;

#[derive(Default, Debug)]
pub struct Ready;

impl BuilderState for BareLattice {}
impl BuilderState for Imported {}
impl BuilderState for ParamSet {}
impl BuilderState for Aligned {}
impl BuilderState for PlaneAdjusted {}
impl BuilderState for Calibrated {}
impl BuilderState for Ready {}

impl ParamSetState for ParamSet {}
impl ParamSetState for Aligned {}
impl ParamSetState for PlaneAdjusted {}
impl ParamSetState for Calibrated {}
impl ParamSetState for Ready {}

#[derive(Debug)]
/// The lifetime `'a` will be from the various `atom_ids` from `AdsInfo`.
pub struct AdsorptionBuilder<'a, T: ModelInfo, U: BuilderState> {
    host_lattice: LatticeModel<T>,
    adsorbate: Option<LatticeModel<T>>,
    location: Option<Point3<f64>>,
    ads_params: Option<AdsParams<'a>>,
    state: PhantomData<U>,
}

#[derive(Debug, Default)]
pub struct AdsParams<'a> {
    ads_direction: Vector3<f64>,
    adsorbate_plane_angle: f64,
    stem_coord_angle: f64,
    bond_length: f64,
    coord_atom_ids: &'a [u32],
    stem_atom_ids: &'a [u32],
    plane_atom_ids: &'a [u32],
}

#[derive(Debug, Default)]
pub struct AdsParamsBuilder<'a, AdsDirSet, CdAngleSet, PlaneAngleSet, BondLengthSet>
where
    AdsDirSet: ToAssign,
    PlaneAngleSet: ToAssign,
    BondLengthSet: ToAssign,
{
    ads_direction: Option<Vector3<f64>>,
    adsorbate_plane_angle: Option<f64>,
    stem_coord_angle: Option<f64>,
    bond_length: Option<f64>,
    coord_atom_ids: Option<&'a [u32]>,
    stem_atom_ids: Option<&'a [u32]>,
    plane_atom_ids: Option<&'a [u32]>,
    ads_direction_set: PhantomData<AdsDirSet>,
    coord_angle_set: PhantomData<CdAngleSet>,
    adsorbate_plane_angle_set: PhantomData<PlaneAngleSet>,
    bond_length_set: PhantomData<BondLengthSet>,
}

impl<'a, AdsDirSet, CdAngleSet, PlaneAngleSet, BondLengthSet>
    AdsParamsBuilder<'a, AdsDirSet, CdAngleSet, PlaneAngleSet, BondLengthSet>
where
    AdsDirSet: ToAssign,
    PlaneAngleSet: ToAssign,
    BondLengthSet: ToAssign,
{
    pub fn new() -> AdsParamsBuilder<'a, No, No, No, No> {
        AdsParamsBuilder {
            ads_direction: None,
            adsorbate_plane_angle: None,
            stem_coord_angle: None,
            bond_length: None,
            coord_atom_ids: None,
            stem_atom_ids: None,
            plane_atom_ids: None,
            ads_direction_set: PhantomData,
            coord_angle_set: PhantomData,
            adsorbate_plane_angle_set: PhantomData,
            bond_length_set: PhantomData,
        }
    }
    pub fn with_ads_direction(
        mut self,
        ads_direction: &Vector3<f64>,
    ) -> AdsParamsBuilder<'a, Yes, CdAngleSet, PlaneAngleSet, BondLengthSet> {
        self.ads_direction = Some(ads_direction.to_owned());
        let Self {
            ads_direction,
            adsorbate_plane_angle,
            stem_coord_angle,
            bond_length,
            coord_atom_ids,
            stem_atom_ids,
            plane_atom_ids,
            ads_direction_set: _,
            coord_angle_set,
            adsorbate_plane_angle_set,
            bond_length_set,
        } = self;
        AdsParamsBuilder {
            ads_direction,
            adsorbate_plane_angle,
            stem_coord_angle,
            bond_length,
            ads_direction_set: PhantomData,
            coord_angle_set,
            adsorbate_plane_angle_set,
            bond_length_set,
            coord_atom_ids,
            stem_atom_ids,
            plane_atom_ids,
        }
    }
    // pub fn with_coord_angle(
    //     mut self,
    //     coord_angle: f64,
    // ) -> AdsParamsBuilder<AdsDirSet, CdAngleSet, Yes, PlaneAngleSet, BondLengthSet> {
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
    ) -> AdsParamsBuilder<'a, AdsDirSet, CdAngleSet, Yes, BondLengthSet> {
        self.adsorbate_plane_angle = Some(plane_angle);
        let Self {
            ads_direction,
            adsorbate_plane_angle,
            stem_coord_angle,
            bond_length,
            ads_direction_set,
            coord_angle_set,
            adsorbate_plane_angle_set: _,
            bond_length_set,
            coord_atom_ids,
            stem_atom_ids,
            plane_atom_ids,
        } = self;
        AdsParamsBuilder {
            ads_direction,
            adsorbate_plane_angle,
            stem_coord_angle,
            bond_length,
            ads_direction_set,
            coord_angle_set,
            adsorbate_plane_angle_set: PhantomData,
            bond_length_set,
            coord_atom_ids,
            stem_atom_ids,
            plane_atom_ids,
        }
    }
    pub fn with_bond_length(
        mut self,
        bond_length: f64,
    ) -> AdsParamsBuilder<'a, AdsDirSet, CdAngleSet, PlaneAngleSet, Yes> {
        self.bond_length = Some(bond_length);
        let Self {
            ads_direction,
            adsorbate_plane_angle,
            stem_coord_angle,
            bond_length,
            ads_direction_set,
            coord_angle_set,
            adsorbate_plane_angle_set,
            bond_length_set: _,
            coord_atom_ids,
            stem_atom_ids,
            plane_atom_ids,
        } = self;
        AdsParamsBuilder {
            ads_direction,
            adsorbate_plane_angle,
            stem_coord_angle,
            bond_length,
            ads_direction_set,
            adsorbate_plane_angle_set,
            coord_angle_set,
            bond_length_set: PhantomData,
            coord_atom_ids,
            stem_atom_ids,
            plane_atom_ids,
        }
    }
    pub fn with_stem_coord_angle(
        mut self,
        coord_angle: f64,
    ) -> AdsParamsBuilder<'a, AdsDirSet, Yes, PlaneAngleSet, BondLengthSet> {
        self.stem_coord_angle = Some(coord_angle);
        let Self {
            ads_direction,
            adsorbate_plane_angle,
            stem_coord_angle,
            bond_length,
            ads_direction_set,
            coord_angle_set: _,
            adsorbate_plane_angle_set,
            bond_length_set,
            coord_atom_ids,
            stem_atom_ids,
            plane_atom_ids,
        } = self;
        AdsParamsBuilder {
            ads_direction,
            adsorbate_plane_angle,
            stem_coord_angle,
            bond_length,
            ads_direction_set,
            coord_angle_set: PhantomData,
            adsorbate_plane_angle_set,
            bond_length_set,
            coord_atom_ids,
            stem_atom_ids,
            plane_atom_ids,
        }
    }
    pub fn with_coord_atom_ids(
        mut self,
        coord_atom_ids: &'a [u32],
    ) -> AdsParamsBuilder<'a, AdsDirSet, CdAngleSet, PlaneAngleSet, BondLengthSet> {
        self.coord_atom_ids = Some(coord_atom_ids);
        self
    }
    pub fn with_stem_atom_ids(
        mut self,
        stem_atom_ids: &'a [u32],
    ) -> AdsParamsBuilder<'a, AdsDirSet, CdAngleSet, PlaneAngleSet, BondLengthSet> {
        self.stem_atom_ids = Some(stem_atom_ids);
        self
    }
    pub fn with_plane_atom_ids(
        mut self,
        plane_atom_ids: &'a [u32],
    ) -> AdsParamsBuilder<'a, AdsDirSet, CdAngleSet, PlaneAngleSet, BondLengthSet> {
        self.plane_atom_ids = Some(plane_atom_ids);
        self
    }
}

impl<'a> AdsParamsBuilder<'a, Yes, Yes, Yes, Yes> {
    pub fn finish(self) -> AdsParams<'a> {
        AdsParams {
            ads_direction: self.ads_direction.unwrap(),
            adsorbate_plane_angle: self.adsorbate_plane_angle.unwrap(),
            stem_coord_angle: self.stem_coord_angle.unwrap(),
            bond_length: self.bond_length.unwrap(),
            coord_atom_ids: self.coord_atom_ids.unwrap(),
            stem_atom_ids: self.stem_atom_ids.unwrap(),
            plane_atom_ids: self.plane_atom_ids.unwrap(),
        }
    }
}

enum StemType {
    RealStem(Vector3<f64>),
    VirtualStem(Vector3<f64>),
}

impl From<StemType> for Vector3<f64> {
    fn from(item: StemType) -> Self {
        match item {
            StemType::RealStem(vec) => vec,
            StemType::VirtualStem(vec) => vec,
        }
    }
}

impl<'a, T, U> AdsorptionBuilder<'a, T, U>
where
    T: ModelInfo,
    U: BuilderState + ParamSetState,
{
    fn get_stem_vector(&self) -> StemType {
        if let Ok(stem_vec) = self
            .adsorbate()
            .get_vector_ab(self.stem_atom_ids()[0], self.stem_atom_ids()[1])
        {
            StemType::RealStem(stem_vec)
        } else {
            let stem_atom_xyz = self
                .adsorbate()
                .get_atom_by_id(self.stem_atom_ids()[0])
                .unwrap()
                .xyz();
            let plane_atom_xyz = self
                .adsorbate()
                .get_atom_by_id(self.plane_atom_ids()[0])
                .unwrap()
                .xyz();
            let plane_normal = self.get_plane_normal();
            let stem_intersects_plane =
                line_plane_intersect(stem_atom_xyz, plane_atom_xyz, &plane_normal, &plane_normal);
            StemType::VirtualStem(stem_intersects_plane - stem_atom_xyz)
        }
    }
    fn get_plane_normal(&self) -> Vector3<f64> {
        let plane_atoms = self.plane_atom_ids();
        plane_normal(
            self.adsorbate()
                .get_atom_by_id(plane_atoms[0])
                .unwrap()
                .xyz(),
            self.adsorbate()
                .get_atom_by_id(plane_atoms[1])
                .unwrap()
                .xyz(),
            self.adsorbate()
                .get_atom_by_id(plane_atoms[2])
                .unwrap()
                .xyz(),
        )
        .unwrap()
    }
    fn move_to_origin(&mut self) {
        let curr_stem_centroid = na::center(
            self.adsorbate()
                .get_atom_by_id(self.stem_atom_ids()[0])
                .unwrap()
                .xyz(),
            self.adsorbate()
                .get_atom_by_id(self.stem_atom_ids()[1])
                .unwrap()
                .xyz(),
        );
        let translate_mat = Translation3::from(Point3::origin() - curr_stem_centroid);
        self.adsorbate_mut().translate(&translate_mat);
    }
    fn flip_up(&mut self, upper_atom_id: u32) {
        let cd_atom_z = self
            .adsorbate()
            .get_atom_by_id(self.coord_atom_ids()[0])
            .unwrap()
            .xyz()
            .z;
        if self
            .adsorbate()
            .get_atom_by_id(upper_atom_id)
            .unwrap()
            .xyz()
            .z
            < cd_atom_z
        {
            // Flip up by 180 degrees
            // The rotation axis is set to be x-axis, so this must be conducted immediately after rolling.
            let rotate_quatd = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), PI);
            self.adsorbate_mut().rotate(&rotate_quatd);
        }
    }
    fn ads_params(&self) -> &AdsParams {
        self.ads_params.as_ref().unwrap()
    }
    fn adsorbate_plane_angle(&self) -> f64 {
        self.ads_params().adsorbate_plane_angle
    }
    fn adsorbate_stem_coord_angle(&self) -> f64 {
        self.ads_params().stem_coord_angle
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
        &self.ads_params().ads_direction
    }
    fn stem_atom_ids(&self) -> &[u32] {
        self.ads_params().stem_atom_ids
    }
    fn coord_atom_ids(&self) -> &[u32] {
        self.ads_params().coord_atom_ids
    }
    fn plane_atom_ids(&self) -> &[u32] {
        self.ads_params().plane_atom_ids
    }
    fn bond_length(&self) -> f64 {
        self.ads_params().bond_length
    }
}

impl<'a, T> AdsorptionBuilder<'a, T, BareLattice>
where
    T: ModelInfo,
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
    ) -> AdsorptionBuilder<'a, T, Imported> {
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

impl<'a, T: ModelInfo> AdsorptionBuilder<'a, T, Imported> {
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
        ads_params: AdsParams<'a>,
    ) -> AdsorptionBuilder<'a, T, ParamSet> {
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

impl<'a, T> AdsorptionBuilder<'a, T, ParamSet>
where
    T: ModelInfo + std::fmt::Debug,
{
    fn check_coordinate(&self) -> Result<(), InvalidCoord> {
        for atom in self.adsorbate().atoms() {
            let xyz = atom.xyz();
            if xyz.x.is_nan() || xyz.y.is_nan() || xyz.z.is_nan() {
                return Err(InvalidCoord());
            }
        }
        Ok(())
    }
    /// "Roll" the plane. The purpose is to lay the specified plane around the stem to the proper angle.
    fn roll_ads(&mut self, upper_atom_id: u32) -> Result<(), InvalidCoord> {
        let ads_atom_nums = self.adsorbate().atoms().len();
        match ads_atom_nums {
            1 => {}
            2 => {}
            _ => {
                let plane_normal = self.get_plane_normal();
                let plane_angle_rad = (90.0 - self.adsorbate_plane_angle()).to_radians();
                let target_angle_vec =
                    Vector3::new(0.0, plane_angle_rad.cos(), plane_angle_rad.sin());
                let roll_quatd =
                    UnitQuaternion::rotation_between(&plane_normal, &target_angle_vec).unwrap();
                self.adsorbate_mut().rotate(&roll_quatd);
            }
        };
        // Flip up check instantly
        self.flip_up(upper_atom_id);
        self.check_coordinate()
    }
    /// Determine the pitch angle. This determines the angle between the stem and target host sites.
    /// The logic for pitch do not differ on type of stem.
    fn pitch_ads(&mut self) -> Result<(), InvalidCoord> {
        let stem_vector: Vector3<f64> = self.get_stem_vector().into();
        let coord_atom_xyz = self
            .adsorbate()
            .get_atom_by_id(self.coord_atom_ids()[0])
            .unwrap()
            .xyz();
        let origin_to_coord = coord_atom_xyz - Point3::origin();
        // Compute dot product of `stem_vector` and `origin_to_coord` to see if they are in same x-direction
        let stem_dot_oc = origin_to_coord.dot(&stem_vector);
        // Make sure pitch up/down towards the coord atom
        let stem_vector = if stem_dot_oc < 0.0 {
            stem_vector * -1.0
        } else {
            stem_vector
        };
        // The x-direction to construct the `coord_dir_vec` --------------------------------|
        let sign = if stem_dot_oc < 0.0 { -1.0 } else { 1.0 }; //                           |
        let stem_vector_xz = Vector3::new(stem_vector.x, 0.0, stem_vector.z).normalize(); //|
        let coord_dir_vec = Vector3::new(
            // Apply the direction sign here <----------------------------------------------|
            sign * self.adsorbate_stem_coord_angle().to_radians().cos(),
            0.0,
            -1.0 * self.adsorbate_stem_coord_angle().to_radians().sin(),
        );
        // Only rotate when the two xz vectors are not collinear
        if (stem_vector_xz.dot(&coord_dir_vec).abs() - 1.0).abs() > 0.001 * f64::EPSILON {
            let pitch_angle = stem_vector_xz.angle(&coord_dir_vec);
            let rot_axis = Unit::new_normalize(stem_vector.cross(&coord_dir_vec));
            let pitch_quatd = UnitQuaternion::from_axis_angle(&rot_axis, pitch_angle);
            self.adsorbate_mut().rotate(&pitch_quatd);
            self.check_coordinate()
        } else {
            Ok(())
        }
    }
    /// Yaw
    fn yaw_ads(&mut self) -> Result<(), InvalidCoord> {
        let stem_vector = self.get_stem_vector();
        match stem_vector {
            StemType::RealStem(stem) => {
                let stem_xy_proj = Vector3::new(stem.x, stem.y, 0.0);
                let dir_xy_proj = Vector3::new(self.ads_direction().x, self.ads_direction().y, 0.0);
                let angle = stem_xy_proj.angle(&dir_xy_proj);
                let rot_axis = Unit::new_normalize(stem_xy_proj.cross(&dir_xy_proj));
                let yaw_quatd = UnitQuaternion::from_axis_angle(&rot_axis, angle);
                self.adsorbate_mut().rotate(&yaw_quatd);
            }
            StemType::VirtualStem(_) => {
                let angle = Vector3::x_axis().xy().angle(&self.ads_direction().xy());
                let yaw_quatd = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), angle);
                self.adsorbate_mut().rotate(&yaw_quatd);
            }
        }
        self.check_coordinate()
    }
    pub fn init_ads(mut self, upper_atom_id: u32) -> AdsorptionBuilder<'a, T, Calibrated> {
        // Use Tate-Bryant convention order for rotation sequence. Unfortunately, nalgebra
        // does not support this order natively. We have to implement the order by ourselves.
        let stem_vector = self.get_stem_vector();
        let rotate_quatd = match stem_vector {
            StemType::RealStem(stem) => {
                UnitQuaternion::rotation_between(&stem, &Vector3::x_axis()).unwrap()
            }
            StemType::VirtualStem(vstem) => {
                UnitQuaternion::rotation_between(&vstem, &Vector3::x_axis()).unwrap()
            }
        };
        self.adsorbate_mut().rotate(&rotate_quatd);
        self.move_to_origin();
        self.roll_ads(upper_atom_id)
            .unwrap_or_else(|e| panic!("{:?} {e} at func: roll_ads", self.adsorbate()));
        self.pitch_ads()
            .unwrap_or_else(|e| panic!("{:?} {e} at func: pitch_ads", self.adsorbate()));
        self.yaw_ads()
            .unwrap_or_else(|e| panic!("{:?} {e} at func: yaw_ads", self.adsorbate()));
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

impl<'a, T: ModelInfo> AdsorptionBuilder<'a, T, Calibrated> {
    /**
    When the adsorbate has single coordination atom, translate the adsorbate
    to a position that the distance between coord site and target site is the
    input bond distance, while the bond direction follows the adsorbate direction.
    */
    fn single_coord(&mut self) {
        let ads = self.adsorbate();
        let location = self.location();
        let coord_atom_id = self.coord_atom_ids()[0];
        let coord_atom_point = ads.get_atom_by_id(coord_atom_id).unwrap().xyz();
        let vertical_proj_from_coord_atom = Vector3::new(0.0, 0.0, self.bond_length());
        // Create a stem_vector guaranteed to be pointing upwards
        let stem_vector = {
            let stem_atom_1 = ads.get_atom_by_id(self.stem_atom_ids()[0]).unwrap();
            let stem_atom_2 = ads.get_atom_by_id(self.stem_atom_ids()[1]).unwrap();
            if stem_atom_2.xyz().z > stem_atom_1.xyz().z {
                stem_atom_2.xyz() - stem_atom_1.xyz()
            } else {
                stem_atom_1.xyz() - stem_atom_2.xyz()
            }
        };
        let angle = stem_vector.angle(&vertical_proj_from_coord_atom);
        let actual_position = if angle - PI / 2.0 > f64::EPSILON {
            println!("Not vertical");
            let unit_stem_vector = Unit::new_normalize(stem_vector);
            let translate_mat = Translation3::from(unit_stem_vector.scale(self.bond_length()));
            translate_mat.transform_point(&location)
        } else {
            Point3::new(location.x, location.y, location.z + self.bond_length())
        };
        // When the coord atom is on the stem
        let translate_mat = Translation3::from(actual_position - coord_atom_point);
        self.adsorbate_mut().translate(&translate_mat);
    }
    /**
    When the adsorbate has multiple coordination atoms, translate
    the adsorbate from centroid of coord atoms to the location (centroid of target sites)
    */
    fn multiple_coord(&mut self) {
        let ads = self.adsorbate();
        let mut location = self.location();
        if self.coord_atom_ids().len() > 1 {
            let coord_atom_points: Vec<&Point3<f64>> = self
                .coord_atom_ids()
                .iter()
                .map(|&coord_id| ads.get_atom_by_id(coord_id).unwrap().xyz())
                .collect();
            let coord_centroid = centroid_of_points(&coord_atom_points);
            location.z += self.bond_length();
            let translate_mat = Translation3::from(location - coord_centroid);
            self.adsorbate_mut().translate(&translate_mat);
        }
    }
    /// Place the adsorbate, depending on the number of coordination atoms.
    /// Transit to `Ready` state.
    pub fn place_adsorbate(mut self) -> AdsorptionBuilder<'a, T, Ready> {
        /*
        When the adsorbate has multiple coordination atoms, translate
        the adsorbate from centroid of coord atoms to the location (centroid of target sites)
        */
        if self.coord_atom_ids().len() > 1 {
            self.multiple_coord();
        } else {
            self.single_coord();
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

impl<'a, T> AdsorptionBuilder<'a, T, Ready>
where
    T: ModelInfo,
{
    pub fn build_adsorbed_lattice(self) -> LatticeModel<T> {
        self.host_lattice + self.adsorbate.unwrap()
    }
}
