use crate::engine_wrapper::{Drawable, MyEngine, Pixel, Position};
use console_engine::Color;

pub struct Tetronomino {
    rotation: Rotation,
    position: Position,
    pixel_positions: [[Position; 4]; 4],
    color: Color,
}
impl Tetronomino {
    pub fn new() -> Tetronomino {
        Tetronomino{
            rotation: Rotation::Quarter,
            position: Position(1, 1),
            pixel_positions: I,
            color: Color::Red
        }
    }
}
impl Drawable for Tetronomino {
    fn draw(&self, engine: &mut MyEngine) {
        for pixel_position in self.pixel_positions[self.rotation as usize] {
            engine.draw(&Pixel::position_color(pixel_position+self.position, self.color));
        }
    }
}

#[derive(Clone, Copy)]
enum Rotation {
    Default = 0,
    Quarter = 1,
    Half = 2,
    ThreeQuarters = 3,
}

pub enum Kind {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

const I: [[Position; 4]; 4] = [
    [Position(-1,0), Position(0,0), Position(1,0), Position(2,0)],
    [Position(0,-2), Position(0,-1), Position(0,0), Position(0,1)],
    [Position(-2,0), Position(-1,0), Position(0,0), Position(1,0)],
    [Position(0,1), Position(0,0), Position(0,-1), Position(0,-2)],
];