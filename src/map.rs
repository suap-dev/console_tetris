use crate::my_engine::{Drawable, MyEngine, Pixel, Vec2};
use console_engine::Color;

pub struct Map {
    width: i32,
    height: i32,
    // pub contents: [Option<Pixel>; 500]
    pub pixels: Vec<Pixel>, 
}
impl Map {
    pub fn empty(width: i32, height: i32) -> Self {
        if width.is_positive() && height.is_positive() {
            #[allow(clippy::cast_sign_loss)]
            let mut pixels = Vec::with_capacity(width as usize * height as usize);
            for i in 0..width {
                for j in 0..height {
                    pixels.push(Pixel::new(Vec2::new(i, j), Color::White));
                }
            }

            Self {
                width,
                height,
                pixels,
            }
        } else {
            panic!("Non positive width ({width}) and height ({height})");
        }
    }
}
impl Drawable for Map {
    // fn draw(&self, engine: &mut MyEngine) {
    //     for i in 0..self.width {
    //         for j in 0..self.height {
    //             engine.set_pixel(&Pixel::new(Vec2::new(i, j), Color::White));
    //         }
    //     }
    // }
    fn get_pixels(&mut self) -> &Vec<Pixel> {
        &self.pixels
    }
}
