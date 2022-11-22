use na::{Matrix3, Point3, RealField, Vector3};

use crate::error::CollinearPoints;

pub fn centroid_of_points(points: &[&Point3<f64>]) -> Point3<f64> {
    let num_points = points.len() as f64;
    let points_sum = points
        .iter()
        .map(|p| -> (f64, f64, f64) { (p.x, p.y, p.z) })
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

pub fn find_perp_vec3<T>(vector: &Vector3<T>) -> Vector3<T>
where
    T: RealField,
{
    let mut a = Matrix3::identity();
    a.set_column(0, vector);
    let mat_a = a.transpose();
    let b = Vector3::new(T::zero(), T::one(), T::one());
    let decomp = mat_a.lu();
    decomp.solve(&b).unwrap()
}

pub fn line_plane_intersect<T>(
    line_point: &Point3<T>,
    plane_point: &Point3<T>,
    plane_normal: &Vector3<T>,
    line_direction: &Vector3<T>,
) -> Point3<T>
where
    T: RealField + Copy,
{
    let line_to_plane: Vector3<T> = line_point - plane_point;
    let prod_1 = line_to_plane.dot(plane_normal);
    let prod_2 = line_direction.dot(plane_normal);
    let scale = prod_1 / prod_2;
    let length = line_direction.scale(scale);
    Point3::<T>::new(
        line_point.x + length.x,
        line_point.y + length.y,
        line_point.z + length.z,
    )
}

pub fn plane_normal<T>(
    point_1: &Point3<T>,
    point_2: &Point3<T>,
    point_3: &Point3<T>,
) -> Result<Vector3<T>, CollinearPoints>
where
    T: RealField,
{
    let va: Vector3<T> = point_2 - point_1;
    let vb: Vector3<T> = point_3 - point_1;
    // Raise Error when the points are collinear
    if va.angle(&vb) == T::pi() || va.angle(&vb) == T::zero() {
        Err(CollinearPoints)
    } else {
        Ok(va.cross(&vb))
    }
}

#[cfg(test)]
mod test {
    use na::{Matrix3, Point3, RowVector3, Vector3};

    use crate::math_helper::find_perp_vec3;

    use super::plane_normal;

    #[test]
    fn test_find_perpendicular_vector() {
        let a = Matrix3::from_rows(&[
            RowVector3::new(3.0, -1.0, -3.0),
            RowVector3::new(0.0, 1.0, 0.0),
            RowVector3::new(0.0, 0.0, 1.0),
        ]);
        let b = Vector3::new(0.0, 1.0, 1.0);
        let decomp = a.lu();
        let x = decomp.solve(&b).unwrap();
        let mut ta: Matrix3<f64> = Matrix3::identity();
        ta.set_column(0, &Vector3::new(3.0, -1.0, -3.0));
        ta.transpose_mut();
        let a = Vector3::new(3.0, -1.0, -3.0);
        let x2 = find_perp_vec3(&a);
        assert_eq!(x, x2);
    }
    #[test]
    fn test_plane_normal() {
        let a = Point3::new(1., 1., 1.);
        let b = Point3::new(2., 2., 2.);
        let c = Point3::new(-1., -1., -1.);
        let result = plane_normal(&a, &b, &c);
        match result {
            Ok(vec) => println!("{}", vec),
            Err(e) => println!("{}", e),
        }
    }
}
