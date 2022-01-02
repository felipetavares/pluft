use speedy2d::dimen::Vector2;

pub fn circle(point: Vector2<f32>) -> f32 {
    let r = 1.0;
    let a = 0.5;
    let b = 1.3;
    let c = 1.0;
    let k = 1.0;

    let x = point.x / 800.0;
    let y = point.y / 800.0;

    // x * x + y * y - r * r

    // point.x - point.y

    let x2 = x * x;
    let y2 = y * y;

    (3.0 * x2 - y2).powi(2) * y2 - (x2 + y2).powi(4)

    // point.y - (point.x / 60.0).sin() * 100.0 - 800.0

    // ((x.powi(2) - y.powi(2) + x).powi(2) - (2.0 * x * y + y).powi(2) + x).powi(2)
    //     + (2.0 * (x.powi(2) - y.powi(2) + x) * (2.0 * x * y + y) + y).powi(2)
    //     - 4.0

    // (x.powi(2) + y.powi(2)).powi(2)
    //     - 2.0 * (1f32).powi(2) * (x.powi(2) - y.powi(2))
    //     - ((1.1f32).powi(4) - (1f32).powi(4))

    // (x + y).sin() - (x * y).cos() + 1.0

    // (x.powi(2) + y.powi(2) - 2.0 * 1.0 * x).powi(2)
    //     - 4.0 * (1.0f32).powi(2) * (x.powi(2) + y.powi(2))

    //let a = -1.0;

    // a * (x - a) * (x.powi(2) + y.powi(2)) - k * x.powi(2)

    // y.powi(4) - x.powi(4) + a * y.powi(2) + b * x.powi(2)

    // a * y.powi(2) - x * (x.powi(2) - 2.0 * b * x + c)

    // let a = 0.5;

    // (x.powi(2) + y.powi(2) + 12.0 * a * x + 9.0 * a.powi(2)).powi(2)
    //     - 4.0 * a * (2.0 * x + 3.0 * a).powi(3)

    // (x.powi(2) + y.powi(2)) * (y.powi(2) + x * (x + a)) - 4.0 * a * x * y.powi(2)

    // x.abs().powf(a) + y.abs().powf(a) - 1.0

    // y - x * (x.powi(2) + y.powi(2)).sqrt().tan()

    // y - x.powf(x)
}
