#![allow(dead_code)]

pub mod adsorbate;
pub mod assemble;
pub mod atom;
pub mod error;
pub mod external_info;
pub mod lattice;
pub mod model_type;
// pub mod param_writer;
pub mod parser;
pub mod test;

extern crate castep_periodic_table as cpt;
extern crate nalgebra as na;

pub trait Transformation {
    fn rotate(&mut self, rotate_quatd: &na::UnitQuaternion<f64>);
    fn translate(&mut self, translate_matrix: &na::Translation<f64, 3>);
}

pub mod math_helper {
    use na::Point3;

    pub fn centroid_of_points(points: &[&Point3<f64>]) -> Point3<f64> {
        let num_points = points.len() as f64;
        let points_xyz: Vec<(f64, f64, f64)> = points
            .iter()
            .map(|p| -> (f64, f64, f64) { (p.x, p.y, p.z) })
            .collect();
        let points_sum = points_xyz
            .into_iter()
            .reduce(|a, b| {
                let (ax, ay, az) = a;
                let (bx, by, bz) = b;
                (ax + bx, ay + by, az + bz)
            })
            .unwrap();
        let (cx, cy, cz) = points_sum;
        Point3::new(cx / num_points, cy / num_points, cz / num_points)
    }
}
