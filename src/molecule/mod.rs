use std::error::Error;

use na::Vector3;

use crate::atom::Atom;

pub mod adsorbate;

pub trait Molecule {
    fn get_atom_by_id(&self, atom_id: u32) -> Result<&Atom, Box<dyn Error>>;
    fn get_mut_atom_by_id(&mut self, atom_id: u32) -> Result<&mut Atom, Box<dyn Error>>;
    fn get_vector_ab(&self, a_id: u32, b_id: u32) -> Result<Vector3<f64>, Box<dyn Error>> {
        let atom_a: &Atom = self.get_atom_by_id(a_id)?;
        let atom_b: &Atom = self.get_atom_by_id(b_id)?;
        let a_xyz = atom_a.xyz();
        let b_xyz = atom_b.xyz();
        Ok(b_xyz - a_xyz)
    }
}
