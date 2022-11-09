use std::fmt::Display;

use crate::atom::Atom;

#[derive(Debug, Clone)]
pub struct MsiFormat;

impl Display for Atom<MsiFormat> {
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
