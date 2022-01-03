use super::calc::intersects_or_converges;
use super::config::*;
use super::tracing::trace;
use super::types::{Implicit, Triangle, UVector, Vector};
use crate::types::Pixel;
use speedy2d::window::MouseButton;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use std::collections::HashSet;

pub struct Plot {
    size_pixels: UVector,

    camera_position: Vector,
    mouse_position: Vector,

    initial_camera_position: Vector,
    click_position: Vector,

    mouse_down: bool,

    curve: Implicit,
}

impl Plot {
    pub fn new(curve: Implicit) -> Plot {
        Plot {
            size_pixels: UVector::new(640, 480),
            camera_position: Vector::new(0.0, 0.0),
            initial_camera_position: Vector::new(0.0, 0.0),
            mouse_position: Vector::new(0.0, 0.0),
            click_position: Vector::new(0.0, 0.0),
            mouse_down: false,
            curve: curve,
        }
    }

    fn plot(&mut self, graphics: &mut Graphics2D, background: bool) {
        let mut filled_pixels = HashSet::<Pixel>::new();

        self.tesselate_triangle(
            graphics,
            self.camera([
                Vector::new(self.size_pixels.x as f32, 0.0),
                Vector::new(self.size_pixels.x as f32, self.size_pixels.y as f32),
                Vector::new(0.0, 0.0),
            ]),
            0,
            background,
            &mut filled_pixels,
        );

        self.tesselate_triangle(
            graphics,
            self.camera([
                Vector::new(0.0, self.size_pixels.y as f32),
                Vector::new(self.size_pixels.x as f32, self.size_pixels.y as f32),
                Vector::new(0.0, 0.0),
            ]),
            0,
            background,
            &mut filled_pixels,
        );

        println!("Filled {} pixels", filled_pixels.len());
    }

    fn tesselate_triangle(
        &mut self,
        graphics: &mut Graphics2D,
        vertices: Triangle,
        depth: u32,
        background: bool,
        pixels: &mut HashSet<Pixel>,
    ) {
        // Draw background
        if background && depth <= MAX_DISPLAY_DEPTH {
            self.draw_triangle(graphics, vertices);
        }

        // Draw curve
        if depth >= MAX_DEPTH {
            if !background {
                let triangle_center = (vertices[0] + vertices[1] + vertices[2]) / 3.0;

                trace(
                    self.curve,
                    triangle_center,
                    self.camera_position,
                    graphics,
                    pixels,
                );
            }

            return;
        }

        // Split the current triangle in two
        let children = [
            [(vertices[1] + vertices[2]) / 2.0, vertices[0], vertices[1]],
            [(vertices[1] + vertices[2]) / 2.0, vertices[0], vertices[2]],
        ];

        for child in children {
            if intersects_or_converges(self.curve, child) || depth <= SEARCH_DEPTH {
                self.tesselate_triangle(graphics, child, depth + 1, background, pixels);
            }
        }
    }

    fn draw_triangle(&self, graphics: &mut Graphics2D, tri: Triangle) {
        graphics.draw_line(
            tri[0] - self.camera_position,
            tri[1] - self.camera_position,
            1.0,
            BLUE,
        );
        graphics.draw_line(
            tri[1] - self.camera_position,
            tri[2] - self.camera_position,
            1.0,
            BLUE,
        );
        graphics.draw_line(
            tri[2] - self.camera_position,
            tri[0] - self.camera_position,
            1.0,
            BLUE,
        );
    }

    fn camera(&self, tri: Triangle) -> Triangle {
        tri.map(|p| p + self.camera_position)
    }
}

impl WindowHandler for Plot {
    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vector) {
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

    fn on_resize(&mut self, helper: &mut WindowHelper, size_pixels: UVector) {
        self.size_pixels = size_pixels;

        self.camera_position = Vector::new(
            -((self.size_pixels.x / 2) as f32),
            -((self.size_pixels.y / 2) as f32),
        );

        helper.request_redraw();
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(WHITE);

        self.plot(graphics, true);
        self.plot(graphics, false);
    }
}
