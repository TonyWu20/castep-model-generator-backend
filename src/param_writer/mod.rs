use glob::glob;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use crate::{cell::CellOutput, external_info::element_table::Element};

pub trait ParamWriter: CellOutput {
    fn generate_all_seed_files<P: AsRef<Path>>(
        element_tab_loc: P,
        target_root_dir: &str,
        potentials_loc: &str,
    ) -> Result<(), Box<dyn Error>>;
    fn to_xsd_scripts(target_root_dir: &str) -> Result<(), Box<dyn Error>> {
        let msi_pattern = format!("{target_root_dir}/**/*.msi");
        let item_collection = glob(&msi_pattern)
            .expect("Failed to read glob pattern")
            .into_iter()
            .par_bridge()
            .into_par_iter()
            .map(|entry| -> Option<String> {
                match entry {
                    Ok(path) => {
                        let stem = path.file_stem().unwrap();
                        let parent = path.parent().unwrap();
                        Some(format!(
                            r#""{}/{}""#,
                            parent.to_str().unwrap(),
                            stem.to_str().unwrap()
                        ))
                    }
                    Err(e) => {
                        println!("glob entry match error: {:?}", e);
                        None
                    }
                }
            })
            .collect::<Vec<Option<String>>>()
            .iter()
            .map(|entry| -> String { entry.as_ref().unwrap().to_string() })
            .collect::<Vec<String>>();
        let all_files_text = item_collection.join(", ");
        let headlines = r#"#!perl
use strict;
use Getopt::Long;
use MaterialsScript qw(:all);
"#;
        let array_text = format!("my @params = (\n{});\n", all_files_text);
        let actions = r#"foreach my $item (@params) {
    my $doc = $Documents{"${item}.msi"};
    $doc->CalculateBonds;
    $doc->Export("${item}.xsd");
    $doc->Save;
    $doc->Close;
}"#;
        let contents = format!("{headlines}{array_text}{actions}");
        fs::write(Path::new("msi_to_xsd.pl"), contents)?;
        Ok(())
    }
    fn write_seed_files(
        &mut self,
        target_root_dir: &str,
        element_infotab: &HashMap<String, Element>,
        potentials_loc: &str,
    ) -> Result<(), Box<dyn Error>>;
    fn export_destination(&self, target_root_dir: &str) -> Result<PathBuf, Box<dyn Error>>;
    fn export_filepath(
        &self,
        target_root_dir: &str,
        filename: &str,
    ) -> Result<PathBuf, Box<dyn Error>>;
    fn get_final_cutoff_energy(
        &self,
        element_infotab: &HashMap<String, Element>,
        potentials_loc: &str,
    ) -> f64;
    fn write_param(
        target_root_dir: &str,
        element_infotab: &HashMap<String, Element>,
        potentials_loc: &str,
    ) -> Result<(), Box<dyn Error>>;
    fn write_kptaux(&self, target_root_dir: &str) -> Result<(), Box<dyn Error>>;
    fn write_trjaux(&self, target_root_dir: &str) -> Result<(), Box<dyn Error>>;
    fn copy_potentials(
        &self,
        target_root_dir: &str,
        element_infotab: &HashMap<String, Element>,
        potentials_loc: &str,
    ) -> Result<(), Box<dyn Error>>;
    fn copy_smcastep_extension(&self, target_root_dir: &str) -> Result<(), Box<dyn Error>>;
    fn write_lsf_script(&self, target_root_dir: &str) -> Result<(), Box<dyn Error>>;
}
