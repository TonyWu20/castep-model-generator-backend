#[cfg(test)]
mod test {
    use crate::assemble::{AdsParamsBuilder, AdsorptionBuilder};
    use castep_core::{
        builder_typestate::No,
        param_writer::{
            castep_param::{BandStructureParam, GeomOptParam},
            seed_writer::SeedWriter,
        },
        CellModel, LatticeModel, MsiModel,
    };
    use nalgebra::Vector3;
    use std::{
        fs::{read_to_string, write},
        str::FromStr,
    };

    #[test]
    fn test_conversion() {
        let test_lat = read_to_string("SAC_GDY_Ag.msi").unwrap();
        let msi_lat: LatticeModel<MsiModel> = LatticeModel::from_str(&test_lat).unwrap();
        let cell_lat: LatticeModel<CellModel> = msi_lat.into();
        let msi_back: LatticeModel<MsiModel> = cell_lat.into();
        write("SAC_GDY_Ag_back.msi", msi_back.msi_export()).unwrap();
    }
    fn build(
        ads_name: &str,
        target_sites: &[u32], // target_sites, plane_angle, coord_angle, stem_atom_ids, coord_atom_ids, plane_atom_ids, stem_name
        plane_angle: Option<f64>,
        coord_angle: Option<f64>,
        stem_atom_ids: Option<&[u32; 2]>,
        coord_atom_ids: &[u32],
        plane_atom_ids: Option<&[u32; 3]>,
        upper_atom_id: u32,
        seed_name: &str,
    ) {
        let test_lat = read_to_string("SAC_GDY_Ag.msi").unwrap();
        let lat = LatticeModel::<MsiModel>::from_str(test_lat.as_str()).unwrap();
        let test_ad = read_to_string(ads_name).unwrap();
        let ads = LatticeModel::<MsiModel>::from_str(test_ad.as_str()).unwrap();
        let ads_direction: Option<Vector3<f64>> = if ads.atoms().size() == 1 {
            None
        } else if target_sites.len() == 1 {
            Some(lat.get_vector_ab(41_u32, 42_u32).unwrap())
        } else {
            Some(lat.get_vector_ab(target_sites[0], target_sites[1]).unwrap())
        };
        let export_loc_str = "test";
        let potential_loc_str = "../C-GDY-SAC/Potentials";
        let builder = AdsorptionBuilder::new(lat);
        println!("Seed: {}", seed_name);
        let built_lattice = builder
            .add_adsorbate(ads)
            .with_location_at_sites(target_sites)
            .with_ads_params(
                AdsParamsBuilder::<No, No, No, No>::new()
                    .with_ads_direction(ads_direction)
                    .with_plane_angle(plane_angle)
                    .with_bond_length(1.4)
                    // .with_stem_coord_angle(45.0) // OCC
                    .with_stem_coord_angle(coord_angle) // CO
                    .with_coord_atom_ids(coord_atom_ids)
                    .with_stem_atom_ids(stem_atom_ids)
                    // .with_plane_atom_ids(&[1, 2, 3]) // OCC
                    .with_plane_atom_ids(plane_atom_ids) // CO
                    .finish(),
            )
            .init_ads(upper_atom_id)
            .place_adsorbate()
            .build_adsorbed_lattice();
        let built_cell: LatticeModel<CellModel> = built_lattice.into();
        let geom_seed_writer: SeedWriter<GeomOptParam> = SeedWriter::build(&built_cell)
            // .with_seed_name("Test_ag_OCC")
            .with_seed_name(seed_name)
            .with_export_loc(export_loc_str)
            .with_potential_loc(potential_loc_str)
            .build();
        geom_seed_writer.write_seed_files().unwrap();
        geom_seed_writer.copy_potentials().unwrap();
        let bs_writer: SeedWriter<BandStructureParam> = geom_seed_writer.into();
        bs_writer.write_seed_files().unwrap();
    }
    #[test]
    fn test_builder() {
        build(
            "OCC.msi",
            &[41],
            Some(0.0),
            Some(50.0),
            Some(&[1, 2]),
            &[1],
            Some(&[1, 2, 3]),
            3,
            "Test_Ag_OCC",
        );
        build(
            "CO.msi",
            &[41],
            None,
            Some(90.0),
            Some(&[1, 2]),
            &[1],
            None,
            2,
            "Test_Ag_CO",
        );
        build(
            "COH.msi",
            &[41],
            Some(90.0),
            Some(90.0),
            Some(&[1, 2]),
            &[1],
            Some(&[1, 2, 3]),
            3,
            "Test_Ag_COH",
        );
        build(
            "CH2.msi",
            &[41],
            Some(90.0),
            Some(0.0),
            Some(&[2, 3]),
            &[1],
            Some(&[1, 2, 3]),
            2,
            "Test_Ag_CH2",
        );
        build(
            "COOH.msi",
            &[41],
            Some(90.0),
            Some(0.0),
            Some(&[2, 3]),
            &[1],
            Some(&[1, 2, 3]),
            4,
            "Test_Ag_COOH",
        );
        build(
            "CH3.msi",
            &[41],
            Some(0.0),
            Some(90.0),
            Some(&[1, 1]),
            &[1],
            Some(&[2, 3, 4]),
            2,
            "Test_Ag_CH3",
        )
    }
    #[test]
    fn occo_cc() {
        build(
            "OCCO_cc.msi",
            &[52, 53],
            Some(90.0),
            Some(0.0),
            Some(&[1, 2]),
            &[1, 2],
            Some(&[1, 2, 3]),
            3,
            "Test_Ag_OCCO_CC_FR_c4",
        );
        build(
            "OCCO_cc.msi",
            &[53, 52],
            Some(90.0),
            Some(0.0),
            Some(&[1, 2]),
            &[1, 2],
            Some(&[1, 2, 3]),
            3,
            "Test_Ag_OCCO_CC_c4_FR",
        )
    }
    #[test]
    fn ch3ch2() {
        build(
            "CH3CH2.msi",
            &[41],
            Some(150.0),
            Some(0.0),
            Some(&[3, 2]),
            &[3],
            Some(&[2, 3, 5]),
            1,
            "Test_Ag_CH3CH2",
        );
        build(
            "CH3CH.msi",
            &[41],
            Some(40.0),
            Some(0.0),
            Some(&[2, 1]),
            &[2],
            Some(&[1, 2, 6]),
            6,
            "Test_Ag_CH3CH",
        )
    }
    #[test]
    fn ch2c_ch2ch() {
        build(
            "CH2CH.msi",
            &[41],
            Some(0.0),
            Some(0.0),
            Some(&[2, 1]),
            &[2],
            Some(&[1, 3, 4]),
            4,
            "Test_Ag_CH2CH",
        );
        build(
            "CH2C.msi",
            &[41],
            Some(0.0),
            Some(0.0),
            Some(&[2, 1]),
            &[2],
            Some(&[1, 3, 4]),
            4,
            "Test_Ag_CH2C",
        );
    }
    #[test]
    fn hydrogen() {
        build(
            "H.msi",
            &[41],
            None,
            Some(0.0),
            None,
            &[1],
            None,
            1,
            "Test_Ag_H",
        )
    }
}
