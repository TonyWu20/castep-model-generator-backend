/// Assemble adsorbate and lattice.
use std::{collections::HashMap, error::Error};

use crate::molecule::adsorbate::AdsorbateTraits;

/// For lattice that can add adsorbate. The adsorbate must implement `AdsorbateTraits` and `Clone`
pub trait AddAdsorbate {
    /// Generate suffix about adsorbate and coordination sites.
    fn append_mol_name<T: AdsorbateTraits + Clone>(
        &mut self,
        ads: &T,
        target_sites: &[u32],
        coord_site_dict: &HashMap<u32, String>,
    ) -> Result<(), Box<dyn Error>>;
    /// Initiate the adsorbate orientation before moving to target positions.
    fn init_ads_direction<T: AdsorbateTraits + Clone>(
        &self,
        ads: &mut T,
        target_sites: &[u32],
        flip_upright: bool,
    ) -> Result<(), Box<dyn Error>>;
    /// Routine to add adsorbate to the lattice.
    fn add_ads<T: AdsorbateTraits + Clone>(
        &mut self,
        ads: &mut T,
        target_sites: &[u32],
        height: f64,
        flip_upright: bool,
        coord_site_dict: &HashMap<u32, String>,
    ) -> Result<(), Box<dyn Error>>;
}
