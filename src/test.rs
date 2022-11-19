#[cfg(test)]
mod test {
    use std::fs::read_to_string;

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
    fn test_builder() {
        let test_lat = read_to_string("SAC_GDY_Ag.msi").unwrap();
        let lat = LatticeModel::<MsiModel>::try_from(test_lat.as_str()).unwrap();
        let test_ad = read_to_string("COOH.msi").unwrap();
        let ads = LatticeModel::<MsiModel>::try_from(test_ad.as_str()).unwrap();
        let carbon_chain_vector = lat.get_vector_ab(41_u32, 42_u32).unwrap();
        let builder = AdsorptionBuilder::new(lat);
        let built_lattice = builder
            .add_adsorbate(ads)
            .with_location_at_sites(&[41])
            .with_ads_params(
                AdsParamsBuilder::<No, No, No>::new()
                    .with_ads_direction(&carbon_chain_vector)
                    .with_plane_angle(90.0)
                    .with_bond_length(1.4)
                    .finish(),
            )
            .align_ads(&[2, 3])
            .init_ads_plane_direction(&[1, 2, 3])
            .place_adsorbate(&[2, 3], &[1], 1.4)
            .build_adsorbed_lattice();
        let built_cell: LatticeModel<CellModel> = built_lattice.into();
        let export_loc_str = "test";
        let potential_loc_str = "../C-GDY-SAC/Potentials";
        let geom_seed_writer: SeedWriter<GeomOptParam> = SeedWriter::build(&built_cell)
            .with_seed_name("Test_ag_cooh")
            .with_export_loc(export_loc_str)
            .with_potential_loc(potential_loc_str)
            .build();
        geom_seed_writer.write_seed_files().unwrap();
        geom_seed_writer.copy_potentials().unwrap();
        let bs_writer: SeedWriter<BandStructureParam> = geom_seed_writer.into();
        bs_writer.write_seed_files().unwrap();
    }
}
