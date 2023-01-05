mod engine_wrapper;
use engine_wrapper::MyEngine;

fn main() {
    let mut eng: MyEngine;
    match MyEngine::init(30, 20, 15) {
       Ok(my_engine) => eng = my_engine,
       Err(e) => panic!("We're having problems initialising the engine: {:?}", e)
    }

    let px = engine_wrapper::Pixel::default();

    for _i in 0..45 {
        eng.draw(&px);

        eng.update_frame();
        eng.wait_frame();
    }
}
