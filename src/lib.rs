#![allow(dead_code)]

pub mod adsorbate;
pub mod assemble;
pub mod atom;
pub mod error;
pub mod external_info;
pub mod lattice;
pub mod math_helper;
pub mod model_type;
pub mod param_writer;
pub mod parser;
pub mod test;

extern crate castep_periodic_table as cpt;
extern crate nalgebra as na;

pub trait Transformation {
    fn rotate(&mut self, rotate_quatd: &na::UnitQuaternion<f64>);
    fn translate(&mut self, translate_matrix: &na::Translation<f64, 3>);
}
