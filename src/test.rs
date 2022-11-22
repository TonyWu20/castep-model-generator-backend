#[cfg(test)]
mod test {
    use std::fs::{read_to_string, write};

    use crate::{
        assemble::{AdsParamsBuilder, AdsorptionBuilder},
        builder_typestate::No,
        lattice::LatticeModel,
        model_type::{cell::CellModel, msi::MsiModel},
        param_writer::{
            castep_param::{BandStructureParam, GeomOptParam},
            seed_writer::SeedWriter,
        },
    };

    #[test]
    fn test_conversion() {
        let test_lat = read_to_string("SAC_GDY_Ag.msi").unwrap();
        let msi_lat: LatticeModel<MsiModel> = LatticeModel::try_from(test_lat.as_str()).unwrap();
        let cell_lat: LatticeModel<CellModel> = msi_lat.into();
        let msi_back: LatticeModel<MsiModel> = cell_lat.into();
        write("SAC_GDY_Ag_back.msi", msi_back.msi_export()).unwrap();
    }
    fn build(
        ads_name: &str,
        target_sites: &[u32], // target_sites, plane_angle, coord_angle, stem_atom_ids, coord_atom_ids, plane_atom_ids, stem_name
        plane_angle: f64,
        coord_angle: f64,
        stem_atom_ids: &[u32],
        coord_atom_ids: &[u32],
        plane_atom_ids: &[u32],
        upper_atom_id: u32,
        seed_name: &str,
    ) {
        let test_lat = read_to_string("SAC_GDY_Ag.msi").unwrap();
        let lat = LatticeModel::<MsiModel>::try_from(test_lat.as_str()).unwrap();
        let test_ad = read_to_string(ads_name).unwrap();
        let ads = LatticeModel::<MsiModel>::try_from(test_ad.as_str()).unwrap();
        let carbon_chain_vector = lat.get_vector_ab(42_u32, 41_u32).unwrap();
        let export_loc_str = "test";
        let potential_loc_str = "../C-GDY-SAC/Potentials";
        let builder = AdsorptionBuilder::new(lat);
        let built_lattice = builder
            .add_adsorbate(ads)
            .with_location_at_sites(target_sites)
            .with_ads_params(
                AdsParamsBuilder::<No, No, No, No>::new()
                    .with_ads_direction(&carbon_chain_vector)
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
            0.0,
            50.0,
            &[1, 2],
            &[1],
            &[1, 2, 3],
            3,
            "Test_Ag_OCC",
        );
        build(
            "CO.msi",
            &[41],
            0.0,
            90.0,
            &[1, 2],
            &[1],
            &[1, 2, 2],
            2,
            "Test_Ag_CO",
        );
        build(
            "COH.msi",
            &[41],
            90.0,
            0.0,
            &[1, 2],
            &[1],
            &[1, 2, 3],
            3,
            "Test_Ag_COH",
        )
    }
}
