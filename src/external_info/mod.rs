/// Deal with yaml parsing to load configurations for tasks.
use std::{collections::HashMap, error::Error, path::Path};

pub mod adsorbate_table;
pub mod project;

/// Trait for adsorbate and element yaml table.
pub trait YamlTable {
    /// Table itself.
    type Table;
    /// Type inside the vec<>
    type TableItem;
    /// Type for the key in `HashMap`
    type HashKey;
    /// Load table from given path.
    fn load_table<P: AsRef<Path>>(filepath: P) -> Result<Self::Table, Box<dyn Error>>;
    /// Hash table item, key type: String
    fn hash_table(&self) -> Result<HashMap<Self::HashKey, Self::TableItem>, Box<dyn Error>>;
}

#[test]
fn test_yaml() -> Result<(), Box<dyn Error>> {
    use crate::external_info::adsorbate_table::AdsTab;
    use crate::external_info::project::load_project_info;
    let project_info = load_project_info("resources/project.yaml")?;
    let ads_table = AdsTab::load_table(project_info.adsorbate_table_loc())?;
    assert!(ads_table.adsorbates().is_some());
    let hash_ads_tab = ads_table.hash_table()?;
    println!("{:?}", project_info);
    let coord_sites_dict = project_info.hash_coord_site();
    println!("{}", coord_sites_dict.get(&41).unwrap());
    Ok(())
}
