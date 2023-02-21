use crate::engine_wrapper::{Drawable, MyEngine, Pixel, Position};
use console_engine::Color;

pub struct Map {
    pub width: i32,
    pub height: i32,
    // pub contents: [Option<Pixel>; 500]
}
impl Drawable for Map {
    fn draw(&self, engine: &mut MyEngine) {
        for i in 0..self.width {
            for j in 0..self.height {
                engine.set_pxl(&Pixel::position_color(Position(i, j), Color::White));
            }
        }
    }
}
