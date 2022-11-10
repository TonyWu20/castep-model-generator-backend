use std::collections::HashSet;

use na::{Matrix3, Vector3};

use crate::{atom::Atom, error::InvalidIndex, model_type::ModelInfo};

#[derive(Debug, Clone)]
pub struct LatticeModel<ModelType: ModelInfo> {
    lattice_vectors: Option<LatticeVectors<ModelType>>,
    atoms: Vec<Atom<ModelType>>,
    model_type: ModelType,
}

impl<ModelType> LatticeModel<ModelType>
where
    ModelType: ModelInfo,
{
    pub fn new(
        lattice_vectors: Option<LatticeVectors<ModelType>>,
        atoms: Vec<Atom<ModelType>>,
        model_type: ModelType,
    ) -> Self {
        Self {
            lattice_vectors,
            atoms,
            model_type,
        }
    }

    /// Returns the lattice vectors of this [`LatticeModel<ModelType>`].
    pub fn lattice_vectors(&self) -> Option<&LatticeVectors<ModelType>> {
        self.lattice_vectors.as_ref()
    }
    pub fn atoms(&self) -> &[Atom<ModelType>] {
        self.atoms.as_ref()
    }

    pub fn model_type(&self) -> &ModelType {
        &self.model_type
    }

    pub fn atoms_mut(&mut self) -> &mut Vec<Atom<ModelType>> {
        &mut self.atoms
    }
    pub fn get_atom_by_id(&self, atom_id: u32) -> Result<&Atom<ModelType>, InvalidIndex> {
        self.atoms().get(atom_id as usize - 1).ok_or(InvalidIndex)
    }
    pub fn get_mut_atom_by_id(
        &mut self,
        atom_id: u32,
    ) -> Result<&mut Atom<ModelType>, InvalidIndex> {
        self.atoms_mut()
            .get_mut(atom_id as usize - 1)
            .ok_or(InvalidIndex)
    }
    pub fn get_vector_ab(&self, a_id: u32, b_id: u32) -> Result<Vector3<f64>, InvalidIndex> {
        let atom_a_xyz = self.get_atom_by_id(a_id)?.xyz();
        let atom_b_xyz = self.get_atom_by_id(b_id)?.xyz();
        Ok(atom_b_xyz - atom_a_xyz)
    }
    pub fn list_element(&self) -> Vec<String> {
        let mut elm_list: Vec<(String, u32)> = vec![];
        elm_list.extend(
            self.atoms()
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

#[derive(Debug, Clone, PartialEq)]
pub struct LatticeVectors<ModelType: ModelInfo> {
    vectors: Matrix3<f64>,
    model_type: ModelType,
}

impl<ModelType> LatticeVectors<ModelType>
where
    ModelType: ModelInfo,
{
    pub fn new(vectors: Matrix3<f64>, model_type: ModelType) -> Self {
        Self {
            vectors,
            model_type,
        }
    }

    pub fn fractional_coord_matrix(&self) -> Matrix3<f64> {
        let lattice_vectors = self.vectors();
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

    pub fn vectors(&self) -> &Matrix3<f64> {
        &self.vectors
    }

    pub fn model_type(&self) -> &ModelType {
        &self.model_type
    }
}
