mod calc;
mod config;
mod functions;
mod plot;
mod tracing;
mod types;

use functions::curve;
use plot::Plot;
use speedy2d::Window;

fn main() {
    let window = Window::new_centered("Pluft", (640, 480)).unwrap();

    window.run_loop(Plot::new(curve));
}
