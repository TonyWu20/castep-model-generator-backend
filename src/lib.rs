#![allow(dead_code)]

use std::collections::HashMap;

use external_info::element_table::Element;
pub mod assemble;
pub mod atom;
pub mod cell;
pub mod error;
pub mod external_info;
pub mod lattice;
pub mod molecule;
pub mod param_writer;
pub mod parser;
pub mod test;

extern crate castep_periodic_table as cpt;
extern crate nalgebra as na;

/// Shared behaviour for structs representing an atom, a molecule and a lattice
pub trait MsiExport {
    fn output_in_msi(&self) -> String;
}
pub trait CellExport {
    fn output_in_cell(&self, element_info: &HashMap<String, Element>) -> String;
}
pub trait Transformation {
    fn rotate(&mut self, rotate_quatd: &na::UnitQuaternion<f64>);
    fn translate(&mut self, translate_matrix: &na::Translation<f64, 3>);
}
