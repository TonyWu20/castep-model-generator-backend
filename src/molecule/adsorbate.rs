use std::error::Error;

use na::Vector3;

use super::Molecule;

pub trait AdsorbateTraits: Molecule {
    fn get_ads_name(&self) -> String;
    fn get_stem_vector(&self) -> Result<Vector3<f64>, Box<dyn Error>>;
    fn get_plane_normal(&self) -> Result<Vector3<f64>, Box<dyn Error>>;
    fn is_face_up(&self) -> Result<bool, Box<dyn Error>>;
    fn make_upright(&mut self) -> Result<(), Box<dyn Error>>;
}
