use speedy2d::dimen::Vector2;

pub type Vector = Vector2<f32>;
pub type UVector = Vector2<u32>;
pub type Point = Vector;
pub type Triangle = [Vector; 3];
pub type Implicit = fn(Vector) -> f32;

#[derive(Hash, Eq, PartialEq)]
pub struct Pixel {
    x: i32,
    y: i32,
}

impl Pixel {
    pub fn new(point: Vector2<f32>) -> Pixel {
        Pixel {
            x: point.x.floor() as i32,
            y: point.y.floor() as i32,
        }
    }
}
