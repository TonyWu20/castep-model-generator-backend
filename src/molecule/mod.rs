use na::Vector3;

use crate::{atom::Atom, error::InvalidIndex};

pub mod adsorbate;

/// Basic trait for a molecule.
pub trait Molecule {
    /**
    Return the immutable reference to `Vec<Atom>` as `&[Atom]` of the molecule.
    Implementation depends on the specific field name in user's struct.
    # Example:
    ```
    pub struct Mol {
        // ... other fields
        atoms: Vec<Atom>
    }
    fn get_atoms(&self) -> &[Atom] {
        self.atoms.as_ref()
    }
    ```
    */
    fn get_atoms(&self) -> &[Atom];
    /**
    Return the mutable reference to `Vec<Atom>` as `&mut Vec<Atom>`.
    */
    fn get_mut_atoms(&mut self) -> &mut Vec<Atom>;
    /**
    Return the immutable reference to an `Atom` as `&Atom`. The `atom_id` is 1-indexed, follows the
    rules in `.msi` format.
    If the `atom_id` is not a valid index of the `Vec<Atom>`, the `InvalidIndex` error will be raised.
    */
    fn get_atom_by_id(&self, atom_id: u32) -> Result<&Atom, InvalidIndex> {
        self.get_atoms()
            .get(atom_id as usize - 1)
            .ok_or_else(|| InvalidIndex.into())
    }
    /**
    Return the mutable reference to an `Atom` as `&mut Atom`. The `atom_id` is 1-indexed, follows the
    rules in `.msi` format.
    If the `atom_id` is not a valid index of the `Vec<Atom>`, the `InvalidIndex` error will be raised.
    */
    fn get_mut_atom_by_id(&mut self, atom_id: u32) -> Result<&mut Atom, InvalidIndex> {
        self.get_mut_atoms()
            .get_mut(atom_id as usize - 1)
            .ok_or_else(|| InvalidIndex.into())
    }
    /**
    Return a `Vector3<f64>` pointing from a to b.
    # Arguments:
    * a_id: `u32` - atom id for the starting point.
    * b_id: `u32` - atom id for the ending of the vector.
    If the `atom_id` is not a valid index of the `Vec<Atom>`, the `InvalidIndex` error will be raised.
    */
    fn get_vector_ab(&self, a_id: u32, b_id: u32) -> Result<Vector3<f64>, InvalidIndex> {
        let atom_a: &Atom = self.get_atom_by_id(a_id)?;
        let atom_b: &Atom = self.get_atom_by_id(b_id)?;
        let a_xyz = atom_a.xyz();
        let b_xyz = atom_b.xyz();
        Ok(b_xyz - a_xyz)
    }
}
