use console_engine::ConsoleEngine;
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
}
