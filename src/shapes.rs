use crate::engine_wrapper::Drawable;
use crate::engine_wrapper::Pixel;
use crate::engine_wrapper::MyEngine;


pub struct Shape{
    pixels: Vec<Pixel>
}
impl Shape {
    pub fn from(pixels: Vec<Pixel>) -> Shape {
        Shape{ pixels }
    }
}
impl Drawable for Shape {    
    fn draw(&self, engine: &mut MyEngine){
        for &pixel in &self.pixels {
            pixel.draw(engine);
        }
    }
}
