use crate::Transformation;
use ::core::fmt;
use std::cmp::Ordering;

use na::Point3;
/// Struct that defines an atom.
#[derive(Debug, Clone)]
pub struct Atom<Format> {
    /// The symbol of the element.
    element_symbol: String,
    /// The atomic number of the element in periodic table.
    element_id: u32,
    /// The cartesian coordinate of the atom.
    xyz: Point3<f64>,
    /// The id of the atom in the parsed model.
    atom_id: u32,
    format: Format,
}

impl<Format> Atom<Format> {
    /// Creates a new [`Atom`].
    pub fn new(
        element_symbol: String,
        element_id: u32,
        xyz: Point3<f64>,
        atom_id: u32,
        format: Format,
    ) -> Self {
        Self {
            element_symbol,
            element_id,
            xyz,
            atom_id,
            format,
        }
    }

    /// Returns a reference to the element symbol of this [`Atom<Format>`].
    pub fn element_symbol(&self) -> &str {
        self.element_symbol.as_ref()
    }
    /// Sets the element symbol of this [`Atom<Format>`].
    pub fn set_element_symbol(&mut self, element_symbol: String) {
        self.element_symbol = element_symbol;
    }

    /// Returns the element id of this [`Atom<Format>`].
    pub fn element_id(&self) -> u32 {
        self.element_id
    }
    /// Sets the element id of this [`Atom<Format>`].
    pub fn set_element_id(&mut self, element_id: u32) {
        self.element_id = element_id;
    }

    /// Returns a reference to the xyz of this [`Atom<Format>`].
    pub fn xyz(&self) -> &Point3<f64> {
        &self.xyz
    }

    /// Sets the xyz of this [`Atom<Format>`].
    pub fn set_xyz(&mut self, xyz: Point3<f64>) {
        self.xyz = xyz;
    }

    /// Returns the atom id of this [`Atom<Format>`].
    pub fn atom_id(&self) -> u32 {
        self.atom_id
    }
    /// Sets the atom id of this [`Atom<Format>`].
    pub fn set_atom_id(&mut self, atom_id: u32) {
        self.atom_id = atom_id;
    }

    /// Returns a reference to the format of this [`Atom<Format>`].
    pub fn format(&self) -> &Format {
        &self.format
    }
    /// Sets the format of this [`Atom<Format>`].
    pub fn set_format(&mut self, format: Format) {
        self.format = format;
    }
}

// impl Export for Vec<Atom> {
//     fn format_output(&self) -> String {
//         let atom_strings: Vec<String> = self.iter().map(|atom| atom.format_output()).collect();
//         atom_strings.concat()
//     }
// }

impl<Format> Transformation for Atom<Format> {
    fn rotate(&mut self, rotate_quatd: &na::UnitQuaternion<f64>) {
        self.set_xyz(rotate_quatd.transform_point(self.xyz()))
    }

    fn translate(&mut self, translate_matrix: &na::Translation<f64, 3>) {
        self.set_xyz(translate_matrix.transform_point(self.xyz()))
    }
}

impl<Format> fmt::Display for Atom<Format> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Element: {}\nElement ID: {}\ncoord: {}\nAtom ID: {}",
            self.element_symbol, self.element_id, self.xyz, self.atom_id
        )
    }
}

impl<Format> Ord for Atom<Format> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.atom_id.cmp(&other.atom_id)
    }
}

impl<Format> PartialOrd for Atom<Format> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Format> PartialEq for Atom<Format> {
    fn eq(&self, other: &Self) -> bool {
        self.atom_id == other.atom_id
    }
}

impl<Format> Eq for Atom<Format> {}
