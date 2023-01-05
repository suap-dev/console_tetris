mod engine_wrapper;
use engine_wrapper::MyEngine;

fn main() {
    let mut eng: MyEngine;
    match MyEngine::init(20, 20, 30) {
       Ok(my_engine) => eng = my_engine,
       Err(e) => panic!("We're having problems initialising the engine: {:?}", e)
    }
}
