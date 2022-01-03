use super::types::{Point, Vector};

fn scale(point: Point) -> Point {
    let scale = 1.0 / 300.0;

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

fn curved_triangle(point: Point) -> f32 {
    let a = 0.5;

    match scale(point) {
        Point { x, y } => {
            (x.powi(2) + y.powi(2) + 12.0 * a * x + 9.0 * a.powi(2)).powi(2)
                - 4.0 * a * (2.0 * x + 3.0 * a).powi(3)
        }
    }
}

fn flower(point: Point) -> f32 {
    match scale(point) * 1.1 {
        Point { x, y } => {
            (3.0 * x.powi(2) - y.powi(2)).powi(2) * y.powi(2) - (x.powi(2) + y.powi(2)).powi(4)
        }
    }
}

pub fn curve(point: Point) -> f32 {
    petals(point) * circle(point) * curved_triangle(point) * flower(point)
}
