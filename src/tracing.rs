use super::calc::{abs_inverse_gradient, dot};
use super::config::{BLACK, MAX_TRACE_PIXELS};
use super::types::Pixel;
use super::types::{Implicit, Point, Vector};
use speedy2d::Graphics2D;
use std::collections::HashSet;

fn closest_rotation_match(a: Vector, b: Vector) -> Vector {
    let rots = [
        b.rotate_90_degrees_clockwise(),
        b.rotate_90_degrees_anticlockwise(),
    ];

    let dots = rots.iter().map(|&rot| dot(rot, a)).collect::<Vec<f32>>();

    if dots[0] > dots[1] {
        rots[0]
    } else {
        rots[1]
    }
}

fn pixel_intersects_curve(pixel: Point, curve: Implicit) -> bool {
    let quad = [
        pixel,
        pixel + Vector::new(1.0, 0.0),
        pixel + Vector::new(1.0, 1.0),
        pixel + Vector::new(0.0, 1.0),
    ];

    let intersects: bool = quad
        .map(|vertex| curve(vertex).signum())
        .iter()
        .sum::<f32>()
        .abs() as u8
        != 4;

    intersects
}

fn find_pixel_on_curve(curve: Implicit, mut p: Point) -> Point {
    let mut search_distance = 20f32;
    let mut current_direction = abs_inverse_gradient(curve, p).unwrap();
    let mut previous_direction = current_direction;

    // FIXME: what if there are local minimas and we never cross the zero?
    for _ in 0..32 {
        p = p + current_direction * search_distance;

        // We found a pixel on the curve! Early exit.
        if pixel_intersects_curve(p, curve) {
            return Point::new(p.x.floor(), p.y.floor());
        }

        previous_direction = current_direction;
        current_direction = abs_inverse_gradient(curve, p).unwrap();

        // We crossed the zero of the function
        if dot(previous_direction, current_direction) < 0.0 {
            search_distance /= 2f32;
        }
    }

    return Point::new(p.x.floor(), p.y.floor());
}

fn find_pixel_in_direction(f: Implicit, point: Point, dir: Vector) -> Option<Point> {
    let pixel = Point::new(point.x.floor(), point.y.floor());

    let all_directions = [
        Vector::new(1.0, 0.0),
        Vector::new(-1.0, 0.0),
        Vector::new(0.0, 1.0),
        Vector::new(0.0, -1.0),
        Vector::new(1.0, 1.0),
        Vector::new(-1.0, -1.0),
        Vector::new(-1.0, 1.0),
        Vector::new(1.0, -1.0),
    ];

    let mut directions = all_directions
        .iter()
        .filter(|candidate| pixel_intersects_curve(pixel + **candidate, f))
        .collect::<Vec<&Vector>>();

    // FIXME: Weird hack for sorting floating point...
    directions.sort_by_key(|candidate| (-dot(dir, **candidate) * 1000.0) as i32);

    match directions.len() {
        0 => None,
        _ => Some(pixel + directions[0].clone()),
    }
}

pub fn trace(
    curve: Implicit,
    start: Point,
    cam: Vector,
    graphics: &mut Graphics2D,
    points: &mut HashSet<Pixel>,
    initial_direction: bool,
) {
    let mut pixel = find_pixel_on_curve(curve, start);

    let mut zero_direction;
    let mut trace_direction = None;

    for _ in 0..MAX_TRACE_PIXELS {
        zero_direction = abs_inverse_gradient(curve, pixel);

        trace_direction = match trace_direction {
            None => Some(match initial_direction {
                false => zero_direction.unwrap().rotate_90_degrees_clockwise(),
                true => zero_direction.unwrap().rotate_90_degrees_anticlockwise(),
            }),
            Some(prev_trace_direction) => Some(closest_rotation_match(
                prev_trace_direction,
                zero_direction.unwrap(),
            )),
        };

        match find_pixel_in_direction(curve, pixel, trace_direction.unwrap()) {
            Some(next_pixel) => {
                pixel = next_pixel;
            }
            None => {
                break;
            }
        }

        let pixel_hash = Pixel::new(pixel);

        // FIXME: stop tracing if pixel is outside the screen
        if points.contains(&pixel_hash) {
            break;
        } else {
            graphics.draw_line(pixel - cam, pixel - cam + Vector::new(1.0, 1.0), 1.0, BLACK);
            points.insert(pixel_hash);
        }
    }
}
