use crate::{MsiExport, Transformation};
use ::core::fmt;
use std::cmp::Ordering;

use na::Point3;
/// Struct that defines an atom.
#[derive(Debug, Clone)]
pub struct Atom {
    /// The symbol of the element.
    element_symbol: String,
    /// The atomic number of the element in periodic table.
    element_id: u32,
    /// The cartesian coordinate of the atom.
    xyz: Point3<f64>,
    /// The id of the atom in the parsed model.
    atom_id: u32,
}

impl Atom {
    /// Creates a new [`Atom`].
    pub fn new(element_symbol: String, element_id: u32, xyz: Point3<f64>, atom_id: u32) -> Self {
        Self {
            element_symbol,
            element_id,
            xyz,
            atom_id,
        }
    }

    /// Returns a reference to the element name of this [`Atom`].
    pub fn element_symbol(&self) -> &str {
        &self.element_symbol
    }
    /// Sets the element name of this [`Atom`].
    pub fn set_element_symbol(&mut self, new_symbol: &str) {
        self.element_symbol = new_symbol.to_string();
    }
    /// Returns the element id of this [`Atom`].
    pub fn element_id(&self) -> u32 {
        self.element_id
    }
    /// Sets the element id of this [`Atom`].
    pub fn set_element_id(&mut self, new_id: u32) {
        self.element_id = new_id;
    }
    /// Returns a reference to the xyz of this [`Atom`].
    pub fn xyz(&self) -> &Point3<f64> {
        &self.xyz
    }
    /// Sets the xyz of this [`Atom`].
    pub fn set_xyz(&mut self, new_xyz: Point3<f64>) {
        self.xyz = new_xyz;
    }
    /// Returns the atom id of this [`Atom`].
    pub fn atom_id(&self) -> u32 {
        self.atom_id
    }

    /// Sets the atom id of this [`Atom`].
    pub fn set_atom_id(&mut self, atom_id: u32) {
        self.atom_id = atom_id;
    }
}

impl MsiExport for Atom {
    fn output_in_msi(&self) -> String {
        let msi_output: String = format!(
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
        );
        msi_output
    }
}

// impl Export for Vec<Atom> {
//     fn format_output(&self) -> String {
//         let atom_strings: Vec<String> = self.iter().map(|atom| atom.format_output()).collect();
//         atom_strings.concat()
//     }
// }

impl Transformation for Atom {
    fn rotate(&mut self, rotate_quatd: &na::UnitQuaternion<f64>) {
        self.set_xyz(rotate_quatd.transform_point(self.xyz()))
    }

    fn translate(&mut self, translate_matrix: &na::Translation<f64, 3>) {
        self.set_xyz(translate_matrix.transform_point(self.xyz()))
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Element: {}\nElement ID: {}\ncoord: {}\nAtom ID: {}",
            self.element_symbol, self.element_id, self.xyz, self.atom_id
        )
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.atom_id.cmp(&other.atom_id)
    }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.atom_id == other.atom_id
    }
}

impl Eq for Atom {}
// End Atom
