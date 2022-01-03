use super::config::SAMPLE_RADIUS;
use super::types::{Implicit, Point, Triangle, Vector};

fn sample_triangle(p: Point) -> Triangle {
    [
        p + Vector::new(0.0, -SAMPLE_RADIUS),
        p + Vector::new(-SAMPLE_RADIUS, SAMPLE_RADIUS),
        p + Vector::new(SAMPLE_RADIUS, SAMPLE_RADIUS),
    ]
}

pub fn dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y
}

// -∇|f(p)|
pub fn abs_inverse_gradient(f: Implicit, p: Point) -> Option<Vector> {
    // Sample around the center of the pixel
    let sample = sample_triangle(p + Vector::new(0.5, 0.5));

    let center = sample
        .iter()
        .fold(Vector::ZERO, |acc, v| acc + v / sample.len() as f32);
    let center_value = f(center).abs();

    let gradient = sample
        .iter()
        .map(|vertex| (vertex - center) * (f(*vertex).abs() - center_value))
        .fold(Vector::ZERO, |acc, v| acc + v);

    //if gradient != Vector::ZERO {
    match (Vector::ZERO - gradient).normalize() {
        Some(vec) => Some(vec),
        None => Some(Vector::ZERO),
    }
    //} else {
    //    None
    //}
}

fn intersects_2_edges(f: Implicit, tri: Triangle) -> bool {
    (f(tri[0]).signum() + f(tri[1]).signum() + f(tri[2]).signum()).abs() as i32 != 3
}

pub fn converges(curve: Implicit, tri: Triangle) -> bool {
    let center = (tri[0] + tri[1] + tri[2]) / 3.0;

    for point in tri {
        let direction = (center - point).normalize().unwrap();

        if (curve(point) - curve(point + direction)) * curve(point).signum() < 0.0 {
            return false;
        }
    }

    return true;
}

pub fn intersects_or_converges(curve: Implicit, tri: Triangle) -> bool {
    intersects_2_edges(curve, tri) || converges(curve, tri)
}

// fn converge_to_faces(tri: Triangle, f: Implicit) -> bool {
//     converge(tri, (tri[1] + tri[2]) / 2.0, f)
//         || converge(tri, (tri[0] + tri[2]) / 2.0, f)
//         || converge(tri, (tri[1] + tri[0]) / 2.0, f)
// }
//
// fn function_intersects(tri: Triangle, f: Implicit) -> bool {
//     let intersects: bool =
//         (f(tri[0]).signum() + f(tri[1]).signum() + f(tri[2]).signum()).abs() as i32 != 3;
//
//     let center = (tri[0] + tri[1] + tri[2]) / 3.0;
//
//     let converges_to_center: bool = converge(tri, center, f);
//
//     intersects || converges_to_center || converge_to_faces(tri, f)
// }
