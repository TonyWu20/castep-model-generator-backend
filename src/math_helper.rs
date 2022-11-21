use na::{Matrix3, Point3, RealField, Vector3};
use num_traits::{One, Zero};

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
    let b = Vector3::new(Zero::zero(), One::one(), One::one());
    let decomp = mat_a.lu();
    decomp.solve(&b).unwrap()
}

#[cfg(test)]
mod test {
    use na::{Matrix3, RowVector3, Vector3};

    use crate::math_helper::find_perp_vec3;

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
}
