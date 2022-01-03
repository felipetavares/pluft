use super::calc::{abs_inverse_gradient, dot};
use super::config::{BLACK, MAX_TRACE_SEARCH};
use super::types::{Implicit, Pixel, Point, Vector};
use speedy2d::Graphics2D;
use std::collections::{HashSet, VecDeque};

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

fn find_pixel_on_curve(curve: Implicit, mut p: Point) -> Option<Point> {
    let mut search_distance = 20f32;
    let mut current_direction = abs_inverse_gradient(curve, p).unwrap();
    let mut previous_direction;

    // FIXME: what if there are local minimas and we never cross the zero?
    for _ in 0..MAX_TRACE_SEARCH {
        p = p + current_direction * search_distance;

        // We found a pixel on the curve! Early exit.
        if pixel_intersects_curve(p, curve) {
            return Some(Point::new(p.x.floor(), p.y.floor()));
        }

        previous_direction = current_direction;
        current_direction = abs_inverse_gradient(curve, p).unwrap();

        // We crossed the zero of the function
        if dot(previous_direction, current_direction) < 0.0 {
            search_distance /= 2f32;
        }
    }

    None
}

fn find_neighbor_pixels(curve: Implicit, point: Point) -> Vec<Point> {
    let pixel = Point::new(point.x.floor(), point.y.floor());

    let all_directions = vec![
        Vector::new(1.0, 0.0),
        Vector::new(-1.0, 0.0),
        Vector::new(0.0, 1.0),
        Vector::new(0.0, -1.0),
        Vector::new(1.0, 1.0),
        Vector::new(-1.0, -1.0),
        Vector::new(-1.0, 1.0),
        Vector::new(1.0, -1.0),
    ];

    all_directions
        .into_iter()
        .filter(|&candidate| pixel_intersects_curve(pixel + candidate, curve))
        .map(|dir| pixel + dir)
        .collect::<Vec<Point>>()
}

pub fn trace(
    curve: Implicit,
    start: Point,
    cam: Vector,
    graphics: &mut Graphics2D,
    points: &mut HashSet<Pixel>,
) {
    let mut queue = VecDeque::new();

    match find_pixel_on_curve(curve, start) {
        Some(pixel) => queue.push_back(pixel),
        None => {}
    }

    while queue.len() > 0 {
        let pixel = queue.pop_front().unwrap();

        let hashable_pixel = Pixel::new(pixel);

        // FIXME: stop tracing if pixel is outside the screen
        if !points.contains(&hashable_pixel) {
            graphics.draw_line(pixel - cam, pixel - cam + Vector::new(1.0, 1.0), 1.0, BLACK);

            points.insert(hashable_pixel);
            queue.append(&mut VecDeque::from(find_neighbor_pixels(curve, pixel)));
        }
    }
}
