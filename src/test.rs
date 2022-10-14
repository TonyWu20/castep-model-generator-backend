#[cfg(test)]
mod test {
    use std::error::Error;
    use std::fs;

    use crate::assemble::AdsAddition;
    use crate::cell::CellOutput;
    use crate::external_info::adsorbate_table::AdsTab;
    use crate::external_info::element_table;
    use crate::external_info::project::load_project_info;
    use crate::external_info::YamlTable;
    use crate::lattice::Lattice;
    use crate::parser::msi_parser::parse_adsorbate;
    use crate::{parser, Export};

    #[test]
    fn test_parse_ads() -> Result<(), Box<dyn Error>> {
        let ads_table = AdsTab::load_table("./resources/ads_table.yaml")?;
        let parent_dir = ads_table.directory().to_string();
        let hash_table = ads_table.hash_table()?;
        let ads_info = hash_table.get("CH2CHOH").unwrap();
        let parsed_ads = parse_adsorbate(ads_info, &parent_dir)?;
        println!("{:?}", parsed_ads.atoms_vec());
        Ok(())
    }
    #[test]
    fn add_ads_to_lat() -> Result<(), Box<dyn Error>> {
        let filename = "./resources/GDY_tri.msi";
        let base_lat: Lattice = parser::msi_parser::parse_lattice(filename)?;
        // change_atom_element(
        //     base_lat.atoms_vec_mut().get_mut_atom_by_id(73).unwrap(),
        //     "Mn",
        //     25,
        // );
        // change_atom_element(
        //     base_lat.atoms_vec_mut().get_mut_atom_by_id(74).unwrap(),
        //     "Mn",
        //     25,
        // );
        // change_atom_element(
        //     base_lat.atoms_vec_mut().get_mut_atom_by_id(75).unwrap(),
        //     "Ni",
        //     28,
        // );
        // // let frac_mat = fractional_coord_matrix(&base_lat);
        // base_lat.update_base_name();
        let ads_table = AdsTab::load_table("./resources/ads_table.yaml")?;
        let parent_dir = ads_table.directory().to_string();
        let hash_table = ads_table.hash_table()?;
        let ads_info = hash_table.get("CH2CHOH").unwrap();
        let mut parsed_ads = parse_adsorbate(ads_info, &parent_dir)?;
        let project_info = load_project_info("./resources/project.yaml")?;
        let coord_site_dict = project_info.hash_coord_site();
        let coord_cases = &project_info.coord_cases()[0];
        let element_table = element_table::ElmTab::load_table("./resources/element_table.yaml")?;
        let element_info = element_table.hash_table()?;
        coord_cases
            .get_cases(false)
            .iter()
            .try_for_each(|case| -> Result<(), Box<dyn Error>> {
                let mut new_lat = base_lat.clone();
                new_lat.add_ads(
                    &mut parsed_ads,
                    case.0,
                    case.1,
                    1.4,
                    false,
                    &coord_site_dict,
                )?;
                let result = new_lat.format_output();
                let cell = new_lat.cell_output(&element_info);
                let cell_name = new_lat.lattice_name();
                fs::write("test.msi", result)?;
                fs::write(format!("{}.cell", cell_name), cell)?;
                Ok(())
            })?;
        Ok(())
    }
}
