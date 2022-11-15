use na::Point3;

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
