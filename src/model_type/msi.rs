use std::fmt::Display;

use crate::{
    atom::Atom,
    lattice::{LatticeModel, LatticeVectors},
};

use super::ModelInfo;

#[derive(Debug, Clone)]
pub struct MsiModel {
    periodic_type: u8,
    space_group: String,
    cry_tolerance: f64,
}

impl MsiModel {
    /// Returns the periodic type of this [`MsiModel`].
    pub fn periodic_type(&self) -> u8 {
        self.periodic_type
    }

    /// Returns a reference to the space group of this [`MsiModel`].
    pub fn space_group(&self) -> &str {
        self.space_group.as_ref()
    }

    /// Returns the cry tolerance of this [`MsiModel`].
    pub fn cry_tolerance(&self) -> f64 {
        self.cry_tolerance
    }
}

impl Default for MsiModel {
    fn default() -> Self {
        Self {
            periodic_type: 100_u8,
            space_group: "1 1".to_string(),
            cry_tolerance: 0.05,
        }
    }
}

impl ModelInfo for MsiModel {}

/// Display trait for `Atom<MsiModel>`
impl Display for Atom<MsiModel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"  ({item_id} Atom
    (A C ACL "{elm_id} {elm}")
    (A C Label "{elm}")
    (A D XYZ ({x:.12} {y:.12} {z:.12}))
    (A I Id {atom_id})
  )
"#,
            item_id = self.atom_id() + 1,
            elm_id = self.element_id(),
            elm = self.element_symbol(),
            x = self.xyz().x,
            y = self.xyz().y,
            z = self.xyz().z,
            atom_id = self.atom_id(),
        )
    }
}

/// Display trait for `LatticeVectors<MsiModel>`
impl Display for LatticeVectors<MsiModel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vector_a = self.vectors().column(0);
        let vector_b = self.vectors().column(1);
        let vector_c = self.vectors().column(2);
        let vector_a_line = format!(
            "  (A D A3 ({:.12} {:.12} {:.12}))\n",
            vector_a.x, vector_a.y, vector_a.z
        );
        let vector_b_line = format!(
            "  (A D B3 ({:.12} {:.12} {:.12}))\n",
            vector_b.x, vector_b.y, vector_b.z
        );
        let vector_c_line = format!(
            "  (A D C3 ({:.12} {:.12} {:.12}))",
            vector_c.x, vector_c.y, vector_c.z
        );
        writeln!(f, "{vector_a_line}{vector_b_line}{vector_c_line}")
    }
}

impl LatticeModel<MsiModel> {
    pub fn msi_export(&self) -> String {
        let atoms_output: Vec<String> = self
            .atoms()
            .iter()
            .map(|atom| format!("{}", atom))
            .collect();
        if let Some(lattice_vectors) = self.lattice_vectors() {
            let headers_vectors: Vec<String> = vec![
                "# MSI CERIUS2 DataModel File Version 4 0\n".to_string(),
                "(1 Model\n".to_string(),
                "  (A I CRY/DISPLAY (192 256))\n".to_string(),
                format!(
                    "  (A I PeriodicType {})\n",
                    self.model_type().periodic_type()
                ),
                format!(
                    "  (A C SpaceGroup \"{}\")\n",
                    self.model_type().space_group()
                ),
                format!("{}", lattice_vectors),
                format!(
                    "  (A D CRY/TOLERANCE {})\n",
                    self.model_type().cry_tolerance()
                ),
            ];
            format!("{}{})", headers_vectors.concat(), atoms_output.concat())
        } else {
            let headers = "# MSI CERIUS2 DataModel File Version 4 0\n(1 Model\n";
            format!("{}{})", headers, atoms_output.concat())
        }
    }
}