pub use super::Vec2;

use console_engine::Color;

pub enum Half {
    Left,
    Right,
}
#[derive(Clone, Copy)]
pub struct Pixel {
    pub bacgkround_color: Color,
    pub characters_color: Color,
    pub characters: [char; 2],
    pub position: Vec2,
}
impl Pixel {
    pub const fn default() -> Self {
        Self {
            bacgkround_color: Color::White,
            characters_color: Color::Black,
            characters: ['[', ']'],
            position: Vec2::new(0, 0),
        }
    }
    pub const fn new(position: Vec2, color: Color) -> Self {
        Self {
            bacgkround_color: color,
            characters_color: color,
            characters: ['[', ']'],
            position,
        }
    }

    pub fn get_half(&self, half: &Half) -> console_engine::pixel::Pixel {
        console_engine::pixel::pxl_fbg(
            self.characters[match half {
                Half::Left => 0,
                Half::Right => 1,
            }],
            self.characters_color,
            self.bacgkround_color,
        )
    }
}
