use serde::{Deserialize, Serialize};

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
pub trait Adsorbate {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AdsInfo {
    name: String,
    #[serde(rename = "coordAtomIds")]
    coord_atom_ids: Vec<u32>,
    #[serde(rename = "stemAtomIds")]
    stem_atom_ids: Vec<u32>,
    #[serde(rename = "planeAtomIds")]
    plane_atom_ids: Vec<u32>,
    vertical: bool,
    #[serde(rename = "bSym")]
    symmetric: bool,
    #[serde(rename = "upperAtomId")]
    upper_atom_id: u32,
    #[serde(rename = "atomNums")]
    atom_nums: u32,
    #[serde(rename = "pathName")]
    path_name: String,
    rotate_axis_atom_id: Option<u32>,
    stem_tilted_angle: Option<f64>,
}

impl AdsInfo {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn coord_atom_ids(&self) -> &[u32] {
        self.coord_atom_ids.as_ref()
    }

    pub fn stem_atom_ids(&self) -> &[u32] {
        self.stem_atom_ids.as_ref()
    }

    pub fn plane_atom_ids(&self) -> &[u32] {
        self.plane_atom_ids.as_ref()
    }

    pub fn vertical(&self) -> bool {
        self.vertical
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

    pub fn rotate_axis_atom_id(&self) -> Option<u32> {
        self.rotate_axis_atom_id
    }

    pub fn stem_tilted_angle(&self) -> Option<f64> {
        self.stem_tilted_angle
    }
}
