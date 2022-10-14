#![allow(dead_code)]
pub mod assemble;
pub mod atom;
pub mod cell;
pub mod external_info;
pub mod lattice;
pub mod molecule;
pub mod param_writer;
pub mod parser;
pub mod test;

extern crate nalgebra as na;

/// Shared behaviour for structs representing an atom, a molecule and a lattice
pub trait Export {
    fn format_output(&self) -> String;
}
pub trait Transformation {
    fn rotate(&mut self, rotate_quatd: na::UnitQuaternion<f64>);
    fn translate(&mut self, translate_matrix: na::Translation<f64, 3>);
}
