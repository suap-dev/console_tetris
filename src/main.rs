mod engine_wrapper;
mod shapes;
use engine_wrapper::{MyEngine, Pixel};
use shapes::Shape;

fn main() {
    let mut eng: MyEngine;
    match MyEngine::init(30, 20, 15) {
       Ok(my_engine) => eng = my_engine,
       Err(e) => panic!("We're having problems initialising the engine: {:?}", e)
    }

    let px: Pixel = Pixel::default();
    let mut shape: Vec<Pixel> = Vec::new();
    shape.push(px);
    let shape: Shape = Shape::from(shape);

    for _i in 0..45 {
        eng.draw(&shape);

        eng.update_frame();
        eng.wait_frame();
    }
}
