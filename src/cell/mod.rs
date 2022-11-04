use crate::lattice::LatticeTraits;

use cpt::{data::ELEMENT_TABLE, element::LookupElement};

/**
Trait to Produce `.cell` from the lattice. Required `LatticeTraits` to be implemented.
*/
pub trait CellOutput: LatticeTraits {
    /// Format the contents as in `cell`
    fn write_block(&self, block: (String, String)) -> String {
        let (block_name, content) = block;
        format!(
            "%BlOCK {}\n{}%ENDBLOCK {}\n\n",
            block_name, content, block_name
        )
    }
    /// Format the lattice vectors
    fn lattice_vector_str(&self) -> (String, String) {
        let vectors = self.get_lattice_vectors();
        let vector_strings: Vec<String> = vectors
            .column_iter()
            .map(|col| format!("{:24.18}{:24.18}{:24.18}\n", col.x, col.y, col.z))
            .collect();
        ("LATTICE_CART".to_string(), vector_strings.concat())
    }
    /// Convert the cartesian coordinates of atoms to fractional coordinates in the lattice.
    /// External information is required to provide spin number when necessary.
    fn positions_str(&self) -> (String, String) {
        assert!(self.is_atoms_sorted());
        let pos_strings: Vec<String> = self
            .get_atoms()
            .iter()
            .map(|atom| {
                let frac_coord = self.fractional_coord_matrix() * atom.xyz();
                let atom_info = ELEMENT_TABLE.get_by_symbol(atom.element_symbol()).unwrap();
                if atom_info.spin > 0 {
                    let line = format!(
                        "{:>3}{:20.16}{:20.16}{:20.16} SPIN={:14.10}\n",
                        atom.element_symbol(),
                        frac_coord[0],
                        frac_coord[1],
                        frac_coord[2],
                        atom_info.spin as f64
                    );
                    line
                } else {
                    let line = format!(
                        "{:>3}{:20.16}{:20.16}{:20.16}\n",
                        atom.element_symbol(),
                        frac_coord[0],
                        frac_coord[1],
                        frac_coord[2],
                    );
                    line
                }
            })
            .collect();
        ("POSITIONS_FRAC".to_string(), pos_strings.concat())
    }
    /**
    K-point list configuration.
    Example:
    BLOCK KPOINTS_LIST
       0.0000000000000000   0.0000000000000000   0.0000000000000000       1.000000000000000
    ENDBLOCK KPOINTS_LIST
    */
    fn kpoints_list_str(&self) -> (String, String) {
        ("KPOINTS_LIST".to_string(), "   0.0000000000000000   0.0000000000000000   0.0000000000000000       1.000000000000000
".to_string())
    }
    /// Other necessary configurations. The default implementation is an example.
    fn misc_str(&self) -> String {
        let options_1: String = format!(
            "FIX_ALL_CELL : true\n\nFIX_COM : false\n{}",
            self.write_block(("IONIC_CONSTRAINTS".to_string(), "".to_string()))
        );
        let external_efield = self.write_block((
            "EXTERNAL_EFIELD".to_string(),
            "    0.0000000000     0.0000000000     0.0000000000\n".to_string(),
        ));
        let external_pressure = self.write_block((
            "EXTERNAL_PRESSURE".to_string(),
            r#"    0.0000000000    0.0000000000    0.0000000000
                    0.0000000000    0.0000000000
                                    0.0000000000
"#
            .to_string(),
        ));
        let mut misc = String::new();
        misc.push_str(&options_1);
        misc.push_str(&external_efield);
        misc.push_str(&external_pressure);
        misc
    }
    /// Lookup the mass of the species from the given hashmap from the external element info table.
    fn species_mass_str(&self) -> (String, String) {
        let element_list = self.list_element();
        let mass_strings: Vec<String> = element_list
            .iter()
            .map(|elm| -> String {
                let mass: f64 = ELEMENT_TABLE.get_by_symbol(elm).unwrap().mass();
                format!("{:>8}{:17.10}\n", elm, mass)
            })
            .collect();
        ("SPECIES_MASS".to_string(), mass_strings.concat())
    }
    /// Lookup the potential files to used from the external element info table.
    fn species_pot_str(&self) -> (String, String) {
        let element_list = self.list_element();
        let pot_strings: Vec<String> = element_list
            .iter()
            .map(|elm| {
                let pot_file = ELEMENT_TABLE.get_by_symbol(elm).unwrap().potential();
                format!("{:>8}  {}\n", elm, pot_file)
            })
            .collect();
        ("SPECIES_POT".to_string(), pot_strings.concat())
    }
    /// Lookup the lcao states of species from the external element info table.
    fn species_lcao_str(&self) -> (String, String) {
        let element_list = self.list_element();
        let lcao_strings: Vec<String> = element_list
            .iter()
            .map(|elm| {
                let lcao_state = ELEMENT_TABLE.get_by_symbol(elm).unwrap().lcao();
                format!("{:>8}{:9}\n", elm, lcao_state)
            })
            .collect();
        ("SPECIES_LCAO_STATES".to_string(), lcao_strings.concat())
    }
    /// The whole process of generating a `.cell` from the lattice.
    fn cell_output(&mut self) -> String {
        if !self.is_atoms_sorted() {
            self.sort_by_atomic_number();
        }
        self.rotate_to_standard_orientation();
        let mut content = String::new();
        let block_lat_vec = self.write_block(self.lattice_vector_str());
        content.push_str(&block_lat_vec);
        let block_pos = self.write_block(self.positions_str());
        content.push_str(&block_pos);
        let block_kpoints_list = self.write_block(self.kpoints_list_str());
        content.push_str(&block_kpoints_list);
        let block_misc = self.misc_str();
        content.push_str(&block_misc);
        let block_mass = self.write_block(self.species_mass_str());
        content.push_str(&block_mass);
        let block_pot = self.write_block(self.species_pot_str());
        content.push_str(&block_pot);
        let block_lcao = self.write_block(self.species_lcao_str());
        content.push_str(&block_lcao);
        content
    }
}
