mod engine_wrapper;
mod tetronominoes;
mod map;

use engine_wrapper::{MyEngine, Pixel};
use tetronominoes::Tetronomino;
use map::Map;


fn main() {
    let mut eng: MyEngine;
    match MyEngine::init(30, 20, 2) {
       Ok(my_engine) => eng = my_engine,
       Err(e) => panic!("We're having problems initialising the engine: {:?}", e)
    }

    let mut tet: Tetronomino = Tetronomino::new();
    let map: Map = Map{height: 10, width:10};

    for _i in 0..10 {

        tet.rot_left();

        eng.clear();

        eng.draw(&map);
        eng.draw(&tet);

        eng.update_frame();
        eng.wait_frame();
    }
}
