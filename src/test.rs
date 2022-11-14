#[cfg(test)]
mod test {
    use std::fs::{read_to_string, write};

    use crate::{
        assemble::AdsorptionBuilder,
        lattice::LatticeModel,
        model_type::{cell::CellModel, msi::MsiModel},
    };

    #[test]
    fn test_adsorption_builder() {
        let test_lat = read_to_string("SAC_GDY_Ag.msi").unwrap();
        let lat = LatticeModel::<MsiModel>::try_from(test_lat.as_str()).unwrap();
        let test_ad = read_to_string("COOH.msi").unwrap();
        let ads = LatticeModel::<MsiModel>::try_from(test_ad.as_str()).unwrap();
        let carbon_chain_vector = lat.get_vector_ab(41_u32, 42_u32).unwrap();
        let builder = AdsorptionBuilder::new(lat);
        let built_lattice = builder
            .add_adsorbate(ads)
            .set_height(1.4)
            .set_coord_angle(0.0)
            .set_location(&[41])
            .set_ads_direction(&carbon_chain_vector)
            .set_adsorbate_plane_angle(0.0)
            .align_ads(&[2, 3])
            .init_ads_plane_direction(&[1, 2, 3])
            .place_adsorbate(&[2, 3], &[1], 1.4)
            .build_adsorbed_lattice();
        println!("{:?}", built_lattice);
        let built_cell: LatticeModel<CellModel> = LatticeModel::<CellModel>::from(built_lattice);
        write("Test_ag_cooh.cell", built_cell.cell_export()).unwrap();
    }
}
