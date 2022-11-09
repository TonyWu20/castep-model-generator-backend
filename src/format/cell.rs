use std::fmt::Display;

use crate::{
    atom::Atom,
    lattice::{LatticeModel, LatticeTraits, LatticeVectors},
};

use cpt::{data::ELEMENT_TABLE, element::LookupElement};

#[derive(Clone, Debug, PartialEq, Eq)]
/// Struct to represent `cell`format.
pub struct CellFormat {
    /// List of k-points. Each k-point has xyz and a weight factor.
    kpoints_list: Vec<[f64; 4]>,
    /// Option in `IONIC_CONSTRAINTS`
    fix_all_cell: bool,
    /// Option in `IONIC_CONSTRAINTS`
    fix_com: bool,
    external_efield: [f64; 3],
    /// The order is `Rxx`, `Rxy`, `Rxz`, `Ryy`, `Ryz`, `Rzz`
    external_pressure: [f64; 6],
}

/// Default `CellFormat` values
impl Default for CellFormat {
    fn default() -> Self {
        Self {
            kpoints_list: vec![[0.0, 0.0, 0.0, 1.0]],
            fix_all_cell: true,
            fix_com: false,
            external_efield: [0.0, 0.0, 0.0],
            external_pressure: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }
}

/// Methods for `CellFormat`
impl CellFormat {
    fn write_block(block: (String, String)) -> String {
        let (block_name, content) = block;
        format!(
            "%BlOCK {}\n{}%ENDBLOCK {}\n\n",
            block_name, content, block_name
        )
    }
}

/// Methods only for `LatticeModel<CellFormat>`
impl LatticeModel<CellFormat> {
    /// Formatted *fractional coordinates*
    fn positions_str(&self) -> String {
        let coords_strings: Vec<String> = self
            .atoms()
            .iter()
            .map(|atom| format!("{}", atom))
            .collect();
        let coords = coords_strings.concat();
        CellFormat::write_block(("POSITIONS_FRAC".to_string(), coords))
    }
    /**
    This data block contains a list of k-points at which the Brillouin zone will be sampled during a self consistent calculation to find the electronic ground state, along with the associated weights
    # Format:
    ```
    %BLOCK KPOINTS_LIST
        R1i     R1j     R1k     R1w
        R2i     R2j     R2k     R2w
        .
        .
        .
    %ENDBLOCK KPOINTS_LIST
    ```
    The first three entries on a line are the fractional positions of the k-point relative to the reciprocal space lattice vectors.
    The final entry on a line is the weight of the k-point relative to the others specified. The sum of the weights must be equal to 1.
    */
    fn kpoints_list_str(&self) -> String {
        let kpoints_list: Vec<String> = self
            .format()
            .kpoints_list
            .iter()
            .map(|kpoint| {
                let [x, y, z, weight] = kpoint;
                format!("{:20.16}{:20.16}{:20.16}{:20.16}\n", x, y, z, weight)
            })
            .collect();
        CellFormat::write_block(("KPOINTS_LIST".to_string(), kpoints_list.concat()))
    }
    /// No constraints. Future: adapt to settings
    fn ionic_constraints(&self) -> String {
        CellFormat::write_block(("IONIC_CONSTRAINTS".to_string(), "".to_string()))
    }
    /// Miscellaneous parameters
    fn misc_options(&self) -> String {
        let fix = format!(
            "FIX_ALL_CELL : {}\n\nFIX_COM : {}\n{}",
            self.format().fix_all_cell,
            self.format().fix_com,
            self.ionic_constraints()
        );
        let [ex, ey, ez] = self.format().external_efield;
        let external_efield = CellFormat::write_block((
            "EXTERNAL_EFIELD".to_string(),
            format!("{:16.10}{:16.10}{:16.10}\n", ex, ey, ez),
        ));
        let [rxx, rxy, rxz, ryy, ryz, rzz] = self.format().external_pressure;
        let external_pressure = CellFormat::write_block((
            "EXTERNAL_PRESSURE".to_string(),
            format!(
                r#"{:16.10}{:16.10}{:16.10}
                {:16.10}{:16.10}
                                    {:16.10}
"#,
                rxx, rxy, rxz, ryy, ryz, rzz
            ),
        ));
        let mut misc = String::new();
        misc.push_str(&fix);
        misc.push_str(&external_efield);
        misc.push_str(&external_pressure);
        misc
    }
    /**
    Species and mass table
    # Example:
    ```
    %BLOCK SPECIES_MASS
           O     15.9989995956
          Al     26.9820003510
          Ti     47.9000015259
          Cs    132.9049987793
    %ENDBLOCK SPECIES_MASS
    ```
    */
    fn species_mass(&self) -> String {
        let element_list = self.list_element();
        let mass_strings: Vec<String> = element_list
            .iter()
            .map(|elm| -> String {
                let mass: f64 = ELEMENT_TABLE.get_by_symbol(elm).unwrap().mass();
                format!("{:>8}{:17.10}\n", elm, mass)
            })
            .collect();
        CellFormat::write_block(("SPECIES_MASS".to_string(), mass_strings.concat()))
    }
    /**
    Species and potential table
    # Example:
    ```
    %BLOCK SPECIES_POT
       O  O_00.usp
      Al  Al_00.usp
      Ti  Ti_00.uspcc
      Cs  Cs_00.usp
    %ENDBLOCK SPECIES_POT
    ```
    */
    fn species_pot_str(&self) -> (String, String) {
        let element_list = self.list_element();
        let pot_strings: Vec<String> = element_list
            .iter()
            .map(|elm| {
                let pot_file = ELEMENT_TABLE.get_by_symbol(elm).unwrap().potential();
                format!("{:>8}  {}\n", elm, pot_file)
            })
            .collect();
        CellFormat::write_block(("SPECIES_POT".to_string(), pot_strings.concat()))
    }
    /**
    This data block defines the size of the LCAO basis set used for population analysis.
    # Example:
    ```
    %BLOCK SPECIES_LCAO_STATES
       O         2
      Al         2
      Ti         3
      Cs         4
    %ENDBLOCK SPECIES_LCAO_STATES
    ```
    */
    fn species_lcao_str(&self) -> (String, String) {
        let element_list = self.list_element();
        let lcao_strings: Vec<String> = element_list
            .iter()
            .map(|elm| {
                let lcao_state = ELEMENT_TABLE.get_by_symbol(elm).unwrap().lcao();
                format!("{:>8}{:9}\n", elm, lcao_state)
            })
            .collect();
        CellFormat::write_block(("SPECIES_LCAO_STATES".to_string(), lcao_strings.concat()))
    }
    pub fn cell_export(&self) -> String {
        let lattice_vector_string = format!("{}", self.lattice_vectors());
        let cell_text = vec![
            lattice_vector_string,
            self.positions_str(),
            self.kpoints_list_str(),
            self.misc_options(),
            self.species_mass(),
            self.species_pot_str(),
            self.species_lcao_str(),
        ];
        cell_text.concat()
    }
}

impl Display for Atom<CellFormat> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let atom_element = self.element_symbol();
        let spin = ELEMENT_TABLE.get_by_symbol(atom_element).unwrap().spin();
        if spin > 0 {
            write!(
                f,
                "{:>3}{:20.16}{:20.16}{:20.16} SPIN={:14.10}\n",
                atom_element,
                self.xyz().x,
                self.xyz().y,
                self.xyz().z,
                spin as f64
            )
        } else {
            write!(
                f,
                "{:>3}{:20.16}{:20.16}{:20.16}\n",
                atom_element,
                self.xyz().x,
                self.xyz().y,
                self.xyz().z
            )
        }
    }
}

impl Display for LatticeVectors<CellFormat> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_vector: Vec<String> = self
            .vectors()
            .column_iter()
            .map(|col| format!("{:24.18}{:24.18}{:24.18}\n", col.x, col.y, col.z))
            .collect();
        let formatted_vector = formatted_vector.concat();
        let output = self
            .format
            .write_block(("LATTICE_CART".to_string(), formatted_vector));
        write!(f, "{}", &output)
    }
}
