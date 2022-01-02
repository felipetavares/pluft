use super::calc::intersects_or_converges;
use super::config::*;
use super::tracing::trace;
use super::types::{Implicit, Point, Triangle};
use crate::types::Pixel;
use speedy2d::dimen::Vector2;
use speedy2d::window::MouseButton;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use std::collections::HashSet;

pub struct Plot {
    size_pixels: Vector2<u32>,

    camera_position: Vector2<f32>,
    mouse_position: Vector2<f32>,

    initial_camera_position: Vector2<f32>,
    click_position: Vector2<f32>,

    mouse_down: bool,

    function: Implicit,
}

impl Plot {
    pub fn new(f: Implicit) -> Plot {
        Plot {
            size_pixels: Vector2::new(640, 480),
            camera_position: Vector2::new(0.0, 0.0),
            initial_camera_position: Vector2::new(0.0, 0.0),
            mouse_position: Vector2::new(0.0, 0.0),
            click_position: Vector2::new(0.0, 0.0),
            mouse_down: false,
            function: f,
        }
    }

    fn plot(&mut self, graphics: &mut Graphics2D, f: Implicit, background: bool) {
        let mut filled_points = HashSet::<Pixel>::new();

        // trace(
        //     f,
        //     Point::ZERO,
        //     self.camera_position,
        //     graphics,
        //     &mut filled_points,
        //     false,
        // );

        self.tesselate_triangle(
            graphics,
            self.camera([
                Vector2::new(self.size_pixels.x as f32, 0.0),
                Vector2::new(self.size_pixels.x as f32, self.size_pixels.y as f32),
                Vector2::new(0.0, 0.0),
            ]),
            0,
            f,
            background,
            &mut filled_points,
        );

        self.tesselate_triangle(
            graphics,
            self.camera([
                Vector2::new(0.0, self.size_pixels.y as f32),
                Vector2::new(self.size_pixels.x as f32, self.size_pixels.y as f32),
                Vector2::new(0.0, 0.0),
            ]),
            0,
            f,
            background,
            &mut filled_points,
        );
    }

    fn tesselate_triangle(
        &mut self,
        graphics: &mut Graphics2D,
        vertices: Triangle,
        depth: u32,
        f: Implicit,
        background: bool,
        points: &mut HashSet<Pixel>,
    ) {
        if background && depth <= MAX_DISPLAY_DEPTH {
            graphics.draw_line(
                vertices[0] - self.camera_position,
                vertices[1] - self.camera_position,
                1.0,
                BLUE,
            );
            graphics.draw_line(
                vertices[1] - self.camera_position,
                vertices[2] - self.camera_position,
                1.0,
                BLUE,
            );
            graphics.draw_line(
                vertices[2] - self.camera_position,
                vertices[0] - self.camera_position,
                1.0,
                BLUE,
            );
        }

        if depth >= MAX_DEPTH {
            if !background {
                let center = (vertices[0] + vertices[1] + vertices[2]) / 3.0;

                for dir in [true, false] {
                    trace(f, center, self.camera_position, graphics, points, dir);
                }
            }

            return;
        }

        let tri_a = [(vertices[1] + vertices[2]) / 2.0, vertices[0], vertices[1]];
        let tri_b = [(vertices[1] + vertices[2]) / 2.0, vertices[0], vertices[2]];

        if intersects_or_converges(f, tri_a) || depth <= SEARCH_DEPTH {
            self.tesselate_triangle(graphics, tri_a, depth + 1, f, background, points);
        }

        if intersects_or_converges(f, tri_b) || depth <= SEARCH_DEPTH {
            self.tesselate_triangle(graphics, tri_b, depth + 1, f, background, points);
        }
    }

    fn camera(&self, tri: Triangle) -> Triangle {
        tri.map(|p| p + self.camera_position)
    }
}

impl WindowHandler for Plot {
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

        self.plot(graphics, self.function, true);
        self.plot(graphics, self.function, false);
    }
}
