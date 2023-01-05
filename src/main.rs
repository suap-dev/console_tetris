mod engine_wrapper;
mod tetronominoes;

use engine_wrapper::{MyEngine, Pixel};
use tetronominoes::Tetronomino;


fn main() {
    let mut eng: MyEngine;
    match MyEngine::init(30, 20, 15) {
       Ok(my_engine) => eng = my_engine,
       Err(e) => panic!("We're having problems initialising the engine: {:?}", e)
    }

    let px: Pixel = Pixel::default();
    let tet: Tetronomino = Tetronomino::new();
    // let mut shape: Vec<Pixel> = Vec::new();
    // shape.push(px);
    // let shape: Shape = Shape::from(shape);

    for _i in 0..45 {
        eng.draw(&px);
        eng.draw(&tet);

        eng.update_frame();
        eng.wait_frame();
    }
}
