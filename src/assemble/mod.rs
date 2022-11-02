/// Assemble adsorbate and lattice.
use std::{collections::HashMap, error::Error};

use crate::molecule::adsorbate::AdsorbateTraits;

/// For lattice that can add adsorbate
pub trait AddAdsorbate<P: AdsorbateTraits + Clone> {
    /// Generate suffix about adsorbate and coordination sites.
    fn append_mol_name(
        &mut self,
        ads: &P,
        target_sites: &[u32],
        coord_site_dict: &HashMap<u32, String>,
    ) -> Result<(), Box<dyn Error>>;
    /// Initiate the adsorbate orientation before moving to target positions.
    fn init_ads_direction(
        &self,
        ads: &P,
        target_sites: &[u32],
        flip_upright: bool,
    ) -> Result<(), Box<dyn Error>>;
    /// Routine to add adsorbate to the lattice.
    fn add_ads(
        &mut self,
        ads: &P,
        target_sites: &[u32],
        height: f64,
        flip_upright: bool,
        coord_site_dict: &HashMap<u32, String>,
    ) -> Result<(), Box<dyn Error>>;
}
