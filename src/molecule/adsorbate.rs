use std::{error::Error, f64::consts::PI};

use na::Vector3;
use nalgebra::{Unit, UnitQuaternion};

use crate::{error::InvalidIndex, Transformation};

use super::Molecule;

/**
Basic trait for molecules that behave as adsorbates.
# Supertrait(s): `Molecule`
*/
pub trait AdsorbateTraits: Molecule + Transformation {
    /// Returns the adsorbate's name as `String`.
    fn get_ads_name(&self) -> String;
    /// Returns the ids for the two atoms that define the stem of the molecule.
    fn get_stem_atom_ids(&self) -> [u32; 2];
    /// Returns the ids for atoms that define the desired plane of the adsorbates.
    fn get_plane_atom_ids(&self) -> [u32; 3];
    /// Returns the vector of the stem of the adsorbate.
    fn get_stem_vector(&self) -> Result<Vector3<f64>, InvalidIndex> {
        Ok(self.get_vector_ab(self.get_stem_atom_ids()[0], self.get_stem_atom_ids()[1])?)
    }
    /// Returns the normal vector of the plane.
    fn get_plane_normal(&self) -> Result<Vector3<f64>, InvalidIndex> {
        let ab = self.get_vector_ab(self.get_plane_atom_ids()[0], self.get_plane_atom_ids()[1])?;
        let ac = self.get_vector_ab(self.get_plane_atom_ids()[0], self.get_plane_atom_ids()[2])?;
        Ok(ab.cross(&ac).normalize())
    }
    /// Methods to determine whether the adsorbate is upward.
    fn is_upward(&self) -> Result<bool, Box<dyn Error>>;
    /**
    Method to turn the adsorbate facing upward. Depending on
    the structure of the adsorbate, it may be the plane facing upwards,
    or the stem vector vertically pointing upwards.
    */
    fn make_upright(&mut self, is_vertical: bool) -> Result<(), Box<dyn Error>> {
        let stem_vector = self.get_stem_vector()?;
        if is_vertical {
            let plane_normal: Vector3<f64> = self.get_plane_normal()?;
            let mut plane_normal_xy_proj = plane_normal.xyz();
            plane_normal_xy_proj.z = 0.0;
            let rotate_angle = plane_normal.angle(&plane_normal_xy_proj);
            let rot_axis = plane_normal.cross(&plane_normal_xy_proj);
            let rot_axis_stem_angle = rot_axis.angle(&stem_vector);
            let rot_quatd = if rot_axis_stem_angle < PI / 2.0 {
                UnitQuaternion::from_axis_angle(&Unit::new_normalize(stem_vector), rotate_angle)
            } else {
                UnitQuaternion::from_axis_angle(
                    &Unit::new_normalize(stem_vector.scale(-1.0)),
                    rotate_angle,
                )
            };
            self.rotate(&rot_quatd);
        } else {
            let z_axis = Vector3::z_axis();
            let angle = stem_vector.angle(&z_axis);
            let rot_axis = Unit::new_normalize(stem_vector.cross(&z_axis));
            let rot_quatd = UnitQuaternion::from_axis_angle(&rot_axis, angle);
            self.rotate(&rot_quatd);
        };
        if self.is_upward()? == false {
            let stem_vector = self.get_stem_vector()?;
            let invert_quatd =
                UnitQuaternion::from_axis_angle(&Unit::new_normalize(stem_vector), PI);
            self.rotate(&invert_quatd);
            Ok(())
        } else {
            Ok(())
        }
    }
}
