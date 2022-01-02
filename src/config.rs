use speedy2d::color::Color;

pub const SAMPLE_RADIUS: f32 = 0.001;

pub const WHITE: speedy2d::color::Color = Color::from_rgb(1.0, 1.0, 1.0);
pub const BLACK: speedy2d::color::Color = Color::from_rgb(0.0, 0.0, 0.0);
pub const BLUE: speedy2d::color::Color = Color::from_rgb(0.83, 0.85, 1.0);

pub const MAX_DEPTH: u32 = 8;
pub const MAX_DISPLAY_DEPTH: u32 = 8;
pub const SEARCH_DEPTH: u32 = 6;

pub const MAX_TRACE_PIXELS: u32 = 16384;
pub const MAX_TRACE_SEARCH: u32 = 32;
