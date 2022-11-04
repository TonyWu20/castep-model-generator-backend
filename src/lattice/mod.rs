use std::collections::HashSet;

use na::{Matrix3, UnitQuaternion, Vector, Vector3};

use crate::{molecule::Molecule, Transformation};

/// Common behaviours that a struct representing a `Lattice` should have.
pub trait LatticeTraits: Molecule {
    /// Returns the lattice name.
    fn get_lattice_name(&self) -> &str;
    /// Returns the lattice vectors in `Matrix3` type.
    fn get_lattice_vectors(&self) -> &Matrix3<f64>;
    /**
    Set the bool field `sorted` in the `Lattice` struct. Your `Lattice` struct must provide this field.
    The implementation depends on the specific name of the field given.
    */
    fn set_atoms_sorted(&mut self, is_sorted: bool);
    /**
    Return if the field `atoms` is sorted.
    The implementation depends on the specific name of the field given.
    */
    fn is_atoms_sorted(&self) -> bool;
    /// Sort the atoms in the order of atomic number.
    fn sort_by_atomic_number(&mut self) {
        let atoms = self.get_mut_atoms();
        atoms.sort_unstable_by(|a, b| a.element_id().cmp(&b.element_id()));
        self.set_atoms_sorted(true);
    }
    /**
    Returns the cartesian coordinate to fractional coordinate matrix.
    It is relevant with the lattice vectors.
    With default implementation.
    */
    fn fractional_coord_matrix(&self) -> Matrix3<f64> {
        let lattice_vectors = self.get_lattice_vectors();
        let vec_a = lattice_vectors.column(0);
        let vec_b = lattice_vectors.column(1);
        let vec_c = lattice_vectors.column(2);
        let len_a: f64 = vec_a.norm();
        let len_b: f64 = vec_b.norm();
        let len_c: f64 = vec_c.norm();
        let (alpha, beta, gamma) = (
            vec_b.angle(&vec_c),
            vec_a.angle(&vec_c),
            vec_a.angle(&vec_b),
        );
        let vol = vec_a.dot(&vec_b.cross(&vec_c));
        let to_cart = Matrix3::new(
            len_a,
            len_b * gamma.cos(),
            len_c * beta.cos(),
            0.0,
            len_b * gamma.sin(),
            len_c * (alpha.cos() - beta.cos() * gamma.cos()) / gamma.sin(),
            0.0,
            0.0,
            vol / (len_a * len_b * gamma.sin()),
        );
        to_cart.try_inverse().unwrap()
    }
    /**
    Rotate the lattice to standard orientation.
    Example:
    When the standard orientation is A-vector aligning with the x-axis,
    the function will conduct the transformation for the atoms in the lattice.
    */
    fn rotate_to_standard_orientation(&mut self) {
        let x_axis: Vector3<f64> = Vector::x();
        let a_vec = self.get_lattice_vectors().column(0);
        let a_to_x_angle = a_vec.angle(&x_axis);
        if a_to_x_angle == 0.0 {
            return;
        }
        let rot_axis = a_vec.cross(&x_axis).normalize();
        let rot_quatd: UnitQuaternion<f64> = UnitQuaternion::new(rot_axis * a_to_x_angle);
        self.get_mut_atoms()
            .iter_mut()
            .for_each(|atom| atom.rotate(&rot_quatd));
    }
    /// List the existing elements in the lattice, sorted by atomic number.
    fn list_element(&self) -> Vec<String> {
        let mut elm_list: Vec<(String, u32)> = vec![];
        elm_list.extend(
            self.get_atoms()
                .iter()
                .map(|atom| (atom.element_symbol().to_string(), atom.element_id()))
                .collect::<Vec<(String, u32)>>()
                .drain(..)
                .collect::<HashSet<(String, u32)>>()
                .into_iter(),
        );
        elm_list.sort_unstable_by(|a, b| {
            let (_, id_a) = a;
            let (_, id_b) = b;
            id_a.cmp(id_b)
        });
        elm_list
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<String>>()
    }
}
