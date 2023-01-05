use console_engine::{ConsoleEngine, pixel};
use std::io::ErrorKind;
use std::result::Result;

pub struct MyEngine {
    c_engine: ConsoleEngine
}
impl MyEngine {
    pub fn init(
        width: u32,
        height: u32,
        target_fps: u32
    ) -> Result<MyEngine, ErrorKind> {        
        match console_engine::ConsoleEngine::init(
            width*2, height, target_fps
        ) {
            Ok(val) => Ok(MyEngine{c_engine: val}),
            Err(e) => Err(e.kind())
        }
    }

    pub fn wait_frame(&mut self) {
        self.c_engine.wait_frame();
    }
    pub fn set_pxl(&mut self, pixel: &Pixel) {
        let x = pixel.position.0;
        let y = pixel.position.1;
        let fg = pixel.fg;
        let bg = pixel.bg;
        let chars = pixel.chars;        

        self.c_engine.set_pxl(
            x, y,
            pixel::pxl_fbg(chars[0], fg, bg)
        );
        self.c_engine.set_pxl(
            x+1, y,
            pixel::pxl_fbg(chars[1], fg, bg)
        );
    }


    pub fn draw(&mut self, entity: &dyn Drawable) {
        entity.draw(self);
    }
    pub fn update_frame(&mut self) {
        self.c_engine.draw();
    }
}

#[derive(Clone, Copy)]
pub struct Pixel{
    pub bg: console_engine::Color,
    pub fg: console_engine::Color,
    pub chars: [char; 2],
    pub position: Position,
}
impl Pixel {
    pub fn default() -> Pixel{
        Pixel{
            bg: console_engine::Color::White,
            fg: console_engine::Color::Black,
            chars: [' ', ' '],
            position: Position(0,0),
        }
    }
}
impl Drawable for Pixel {
    fn draw(&self, engine: &mut MyEngine) {
        engine.set_pxl(self);
    }
}

#[derive(Clone, Copy)]
pub struct Position(i32, i32);

pub trait Drawable {
    fn draw(&self, engine: &mut MyEngine);
}


    