use console_engine::{pixel, Color, ConsoleEngine};
use std::io::ErrorKind;
use std::result::Result;
use std::ops;

pub struct MyEngine {
    c_engine: ConsoleEngine,
}
impl MyEngine {
    pub fn init(width: u32, height: u32, target_fps: u32) -> Result<MyEngine, ErrorKind> {
        match ConsoleEngine::init(width * 2, height, target_fps) {
            Ok(val) => Ok(MyEngine { c_engine: val }),
            Err(e) => Err(e.kind()),
        }
    }

    pub fn clear(&mut self) {
        self.c_engine.clear_screen();
    }
    pub fn wait_frame(&mut self) {
        self.c_engine.wait_frame();
    }
    pub fn set_pxl(&mut self, pixel: &Pixel) {
        let x = pixel.position.0 * 2;
        let y = pixel.position.1;
        let fg = pixel.fg;
        let bg = pixel.bg;
        let chars = pixel.chars;

        self.c_engine
            .set_pxl(x, y, pixel::pxl_fbg(chars[0], fg, bg));
        self.c_engine
            .set_pxl(x + 1, y, pixel::pxl_fbg(chars[1], fg, bg));
    }

    pub fn draw(&mut self, drawable: &dyn Drawable) {
        drawable.draw(self);
    }
    pub fn update_frame(&mut self) {
        self.c_engine.draw();
    }
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub bg: Color,
    pub fg: Color,
    pub chars: [char; 2],
    pub position: Position,
}
impl Pixel {
    pub fn default() -> Pixel {
        Pixel {
            bg: Color::White,
            fg: Color::Black,
            chars: ['[', ']'],
            position: Position(0, 0),
        }
    }
    pub fn position_color(position: Position, color: Color) -> Pixel {
        Pixel {
            bg: color,
            fg: color,
            chars: ['[', ']'],
            position,
        }
    }
}
impl Drawable for Pixel {
    fn draw(&self, engine: &mut MyEngine) {
        engine.set_pxl(self);
    }
}

#[derive(Clone, Copy)]
pub struct Position(pub i32, pub i32);
impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position(self.0+rhs.0, self.1+rhs.1)
    }
}

pub trait Drawable {
    fn draw(&self, engine: &mut MyEngine);
}
