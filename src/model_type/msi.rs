use std::fmt::Display;

use crate::{atom::Atom, lattice::LatticeVectors};

use super::ModelInfo;

#[derive(Debug, Clone, Default)]
pub struct MsiModel;

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
            "  (A D C3 ({:.12} {:.12} {:.12}))\n",
            vector_c.x, vector_c.y, vector_c.z
        );
        write!(f, "{vector_a_line}{vector_b_line}{vector_c_line}")
    }
}
