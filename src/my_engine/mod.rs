pub(crate) mod pixel;
pub(crate) mod shape;
pub(crate) mod vec2;

pub use pixel::Pixel;
pub use vec2::Vec2;

use std::io::ErrorKind;
use std::result::Result;

use console_engine::{Color, ConsoleEngine};

use crate::my_engine::shape::Shape;

pub struct MyEngine {
    c_engine: ConsoleEngine,
}
impl MyEngine {
    pub fn init(width: u32, height: u32, target_fps: u32) -> Result<Self, ErrorKind> {
        match ConsoleEngine::init(width * 2, height, target_fps) {
            Ok(val) => Ok(Self { c_engine: val }),
            Err(e) => Err(e.kind()),
        }
    }

    pub fn set_pixel(&mut self, pixel: &pixel::Pixel) {
        let (x, y) = (pixel.position.x * 2, pixel.position.y);

        self.c_engine
            .set_pxl(x, y, pixel.get_half(&pixel::Half::Left));
        self.c_engine
            .set_pxl(x + 1, y, pixel.get_half(&pixel::Half::Right));
    }

    pub fn set_pixels(&mut self, pixels: &Vec<pixel::Pixel>) {
        for pixel in pixels {
            self.set_pixel(pixel);
        }
    }

    pub fn set_shape<T: Shape>(&mut self, shape: &T) {
        let origin = shape.get_origin();
        let segments = shape.get_segments();

        for segment in segments {
            let (mut x, y) = ((segment.x + origin.x) * 2, segment.y + origin.y);
            let pixel = console_engine::pixel::pxl_fbg(' ', Color::Red, Color::Red);
            self.c_engine.set_pxl(x, y, pixel);
            x += 1;
            self.c_engine.set_pxl(x, y, pixel);
        }
    }

    // pub fn set_pixels2(&mut self, positions: &Vec<Vec2>, color: Color) {
    //     for pixel in pixels {
    //         self.set_pixel(pixel)
    //     }
    // }

    pub fn clear_canvas(&mut self) {
        self.c_engine.clear_screen();
    }

    pub fn process_input(&mut self) {
        self.c_engine.wait_frame();
    }

    pub fn draw(&mut self) {
        self.c_engine.draw();
    }
}

pub trait Drawable {
    fn get_pixels(&mut self) -> &Vec<Pixel>;
}
