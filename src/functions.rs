use super::types::{Point, Vector};

fn scale(point: Point) -> Point {
    let scale = 1.0 / 200.0;

    point * scale
}

fn circle(point: Point) -> f32 {
    let r = 1.0;

    match scale(point) {
        Point { x, y } => x * x + y * y - r * r,
    }
}

fn petals(point: Point) -> f32 {
    let a = 0.8;

    match scale(point) {
        Point { x, y } => {
            (x.powi(2) + y.powi(2)) * (y.powi(2) + x * (x + a)) - 4.0 * a * x * y.powi(2)
        }
    }
}

pub fn curve(point: Point) -> f32 {
    petals(point + Vector::new(-200.0, 0.0)).min(circle(point))
}
