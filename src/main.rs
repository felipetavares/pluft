use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::MouseButton;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

struct Pluft {
    size_pixels: Vector2<u32>,
    camera_position: Vector2<f32>,
    initial_camera_position: Vector2<f32>,
    mouse_position: Vector2<f32>,
    click_position: Vector2<f32>,
    mouse_down: bool,
}

const WHITE: speedy2d::color::Color = Color::from_rgb(1.0, 1.0, 1.0);
const BLACK: speedy2d::color::Color = Color::from_rgb(0.0, 0.0, 0.0);
const BLUE: speedy2d::color::Color = Color::from_rgb(0.83, 0.85, 1.0);

const MAX_DEPTH: u32 = 21;
const MAX_DISPLAY_DEPTH: u32 = 14;
const SEARCH_DEPTH: u32 = 8;

type Triangle = [Vector2<f32>; 3];
type Implicit = fn(Vector2<f32>) -> f32;

fn circle(point: Vector2<f32>) -> f32 {
    // let r = 500.0;

    // let x = 800.0 - point.x;
    // let y = 800.0 - point.y;

    // x * x + y * y - r * r

    // point.x - point.y

    // let x = (point.x - 700.0) / 1200.0;
    // let y = (point.y - 600.0) / 1200.0;

    // let x2 = x * x;
    // let y2 = y * y;

    // (3.0 * x2 - y2).powi(2) * y2 - (x2 + y2).powi(4)

    // point.y - (point.x / 60.0).sin() * 100.0 - 800.0
    let x = point.x / 100.0;
    let y = point.y / 100.0;

    // ((x.powi(2) - y.powi(2) + x).powi(2) - (2.0 * x * y + y).powi(2) + x).powi(2)
    //     + (2.0 * (x.powi(2) - y.powi(2) + x) * (2.0 * x * y + y) + y).powi(2)
    //     - 4.0

    //   (x.powi(2) + y.powi(2)).powi(2)
    //       - 2.0 * (1f32).powi(2) * (x.powi(2) - y.powi(2))
    //       - ((1.1f32).powi(4) - (1f32).powi(4))

    // (x + y).sin() - (x * y).cos() + 1.0

    // (x.powi(2) + y.powi(2) - 2.0 * 1.0 * x).powi(2)
    //     - 4.0 * (1.0f32).powi(2) * (x.powi(2) + y.powi(2))

    //let a = -1.0;
    //let k = 1.0;

    // a * (x - a) * (x.powi(2) + y.powi(2)) - k * x.powi(2)

    // y.powi(4) - x.powi(4) + a * y.powi(2) + b * x.powi(2)

    // let a = 2.0;
    // let b = 1.3;
    // let c = 1.0;

    // a * y.powi(2) - x * (x.powi(2) - 2.0 * b * x + c)

    // let a = 0.5;

    // (x.powi(2) + y.powi(2) + 12.0 * a * x + 9.0 * a.powi(2)).powi(2)
    //     - 4.0 * a * (2.0 * x + 3.0 * a).powi(3)

    // (x.powi(2) + y.powi(2)) * (y.powi(2) + x * (x + a)) - 4.0 * a * x * y.powi(2)

    // x.abs().powf(a) + y.abs().powf(a) - 1.0

    y - x * (x.powi(2) + y.powi(2)).sqrt().tan()
}

fn function_intersects(tri: Triangle, f: Implicit) -> bool {
    let intersects: bool =
        (f(tri[0]).signum() + f(tri[1]).signum() + f(tri[2]).signum()).abs() as i32 != 3;

    intersects
}

impl WindowHandler for Pluft {
    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vector2<f32>) {
        if self.mouse_down {
            self.camera_position =
                self.initial_camera_position + (self.click_position - self.mouse_position);
        }

        self.mouse_position = position;

        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper, _button: MouseButton) {
        self.click_position = self.mouse_position;
        self.initial_camera_position = self.camera_position;
        self.mouse_down = true;
    }

    fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper, _button: MouseButton) {
        self.mouse_down = false;
    }

    fn on_resize(&mut self, helper: &mut WindowHelper, size_pixels: Vector2<u32>) {
        self.size_pixels = size_pixels;
        helper.request_redraw();
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(WHITE);

        self.plot(graphics, circle, true);
        self.plot(graphics, circle, false);
    }
}

impl Pluft {
    fn plot(&mut self, graphics: &mut Graphics2D, f: Implicit, background: bool) {
        self.tesselate_triangle(
            graphics,
            [
                Vector2::new(self.size_pixels.x as f32, 0.0),
                Vector2::new(self.size_pixels.x as f32, self.size_pixels.y as f32),
                Vector2::new(0.0, 0.0),
            ],
            0,
            f,
            background,
        );

        self.tesselate_triangle(
            graphics,
            [
                Vector2::new(0.0, self.size_pixels.y as f32),
                Vector2::new(self.size_pixels.x as f32, self.size_pixels.y as f32),
                Vector2::new(0.0, 0.0),
            ],
            0,
            f,
            background,
        );
    }

    fn tesselate_triangle(
        &mut self,
        graphics: &mut Graphics2D,
        vertices: Triangle,
        depth: u32,
        f: Implicit,
        background: bool,
    ) {
        if background && depth < MAX_DISPLAY_DEPTH {
            graphics.draw_line(vertices[0], vertices[1], 1.0, BLUE);
            graphics.draw_line(vertices[1], vertices[2], 1.0, BLUE);
            graphics.draw_line(vertices[2], vertices[0], 1.0, BLUE);
        }

        if depth > MAX_DEPTH {
            if !background {
                let center = (vertices[0] + vertices[1] + vertices[2]) / 3.0;
                graphics.draw_line(center, center + Vector2::new(1.0, 1.0), 1.0, BLACK);
            }

            return;
        }

        let tri_a = [(vertices[1] + vertices[2]) / 2.0, vertices[0], vertices[1]];
        let tri_b = [(vertices[1] + vertices[2]) / 2.0, vertices[0], vertices[2]];

        if function_intersects(self.camera(tri_a), f) || depth < SEARCH_DEPTH {
            self.tesselate_triangle(graphics, tri_a, depth + 1, f, background);
        }

        if function_intersects(self.camera(tri_b), f) || depth < SEARCH_DEPTH {
            self.tesselate_triangle(graphics, tri_b, depth + 1, f, background);
        }
    }

    fn camera(&mut self, tri: Triangle) -> Triangle {
        tri.map(|p| p + self.camera_position)
    }
}

fn main() {
    let window = Window::new_centered("Pluft", (640, 480)).unwrap();

    window.run_loop(Pluft {
        size_pixels: Vector2::new(640, 480),
        camera_position: Vector2::new(0.0, 0.0),
        initial_camera_position: Vector2::new(0.0, 0.0),
        mouse_position: Vector2::new(0.0, 0.0),
        click_position: Vector2::new(0.0, 0.0),
        mouse_down: false,
    });
}
