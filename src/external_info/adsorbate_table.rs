/// Parse adsorbate info `yaml` file to get necessary information for loading adsorbate models.
extern crate serde;
use std::{collections::HashMap, error::Error, ops::Deref, path::Path};

use serde::{Deserialize, Serialize};

use crate::adsorbate::AdsInfo;

use super::YamlTable;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AdsTab {
    source_directory: String,
    target_directory: Option<String>,
    #[serde(rename = "Adsorbates")]
    adsorbates: Option<Vec<AdsInfo>>,
}

impl AdsTab {
    pub fn adsorbates(&self) -> Option<&Vec<AdsInfo>> {
        self.adsorbates.as_ref()
    }

    pub fn source_directory(&self) -> &str {
        self.source_directory.as_ref()
    }

    pub fn target_directory(&self) -> Option<&String> {
        self.target_directory.as_ref()
    }
}

impl YamlTable for AdsTab {
    type Table = AdsTab;

    type TableItem = AdsInfo;

    type HashKey = String;

    fn load_table<P: AsRef<Path>>(filepath: P) -> Result<Self::Table, Box<dyn Error>> {
        let ads_table_src = std::fs::File::open(filepath)?;
        let ads_table = serde_yaml::from_reader(ads_table_src)?;
        Ok(ads_table)
    }

    fn hash_table(
        &self,
    ) -> Result<std::collections::HashMap<String, Self::TableItem>, Box<dyn Error>> {
        let mut hash_tab: HashMap<String, AdsInfo> = HashMap::new();
        let adsinfo_vec: &Vec<AdsInfo> = self.adsorbates.as_ref().unwrap();
        adsinfo_vec.iter().for_each(|ads: &AdsInfo| {
            hash_tab.insert(ads.name().to_string(), ads.deref().clone());
        });
        Ok(hash_tab)
    }
}
