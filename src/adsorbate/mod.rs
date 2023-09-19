use castep_model_core::{LatticeModel, ModelInfo};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/**
When defining your custom adsorbate struct, it is suggested to doing like this:
```
pub struct MyAdsorbate<T> {
    lattice_model: LatticeModel<T>,
    adsorbate_info: AdsInfo,
}

impl<T> Adsorbate for MyAdsorbate<T>{}
```
*/

pub struct Single;
pub struct Double;
pub struct Multi;

pub trait AtomNum {}
pub trait CoordNum {}
pub trait Pathway {}

impl AtomNum for Single {}
impl AtomNum for Double {}
impl AtomNum for Multi {}
impl CoordNum for Single {}
impl CoordNum for Double {}
impl CoordNum for Multi {}

#[derive(Debug)]
pub struct Adsorbate<T: ModelInfo, P: Pathway, M: AtomNum, N: CoordNum> {
    lattice_model: LatticeModel<T>,
    ads_info: AdsInfo,
    pathway: PhantomData<P>,
    atom_num: PhantomData<M>,
    coord_num: PhantomData<N>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AdsInfo {
    name: String,
    #[serde(rename = "coordAtomIds")]
    coord_atom_ids: Vec<u32>, // Unknown size
    #[serde(rename = "stemAtomIds")]
    stem_atom_ids: Option<[u32; 2]>, // Only can be an array with a size of 2
    #[serde(rename = "planeAtomIds")]
    plane_atom_ids: Option<[u32; 3]>, // Only can be an array with a size of 3
    plane_angle: Option<f64>,
    stem_angle_at_coord: Option<f64>,
    #[serde(rename = "bondLength")]
    bond_length: Option<f64>,
    #[serde(rename = "bSym")]
    symmetric: bool,
    #[serde(rename = "upperAtomId")]
    upper_atom_id: u32,
    #[serde(rename = "atomNums")]
    atom_nums: u32,
    #[serde(rename = "pathName")]
    path_name: String,
}

impl AdsInfo {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn coord_atom_ids(&self) -> &[u32] {
        self.coord_atom_ids.as_ref()
    }

    pub fn stem_atom_ids(&self) -> Option<&[u32; 2]> {
        self.stem_atom_ids.as_ref()
    }

    pub fn symmetric(&self) -> bool {
        self.symmetric
    }

    pub fn upper_atom_id(&self) -> u32 {
        self.upper_atom_id
    }

    pub fn atom_nums(&self) -> u32 {
        self.atom_nums
    }

    pub fn path_name(&self) -> &str {
        self.path_name.as_ref()
    }

    pub fn plane_angle(&self) -> Option<f64> {
        self.plane_angle
    }

    pub fn stem_angle_at_coord(&self) -> Option<f64> {
        self.stem_angle_at_coord
    }

    pub fn plane_atom_ids(&self) -> Option<&[u32; 3]> {
        self.plane_atom_ids.as_ref()
    }

    pub fn set_plane_angle(&mut self, plane_angle: Option<f64>) {
        self.plane_angle = plane_angle;
    }

    pub fn bond_length(&self) -> Option<f64> {
        self.bond_length
    }
}
