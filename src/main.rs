mod engine_wrapper;
use console_engine::ConsoleEngine;
use engine_wrapper::MyEngine;

fn main() {
    let mut eng: MyEngine;
    match MyEngine::init(30, 20, 15) {
       Ok(my_engine) => eng = my_engine,
       Err(e) => panic!("We're having problems initialising the engine: {:?}", e)
    }
    for _i in 0..20 {
        eng.wait_frame();
    }
}
