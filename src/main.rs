#![warn(clippy::pedantic)]
#![allow(dead_code, unused_imports)]

mod map;
mod my_engine;
mod tetronominoes;

use console_engine::Color;
use map::Map;
use my_engine::MyEngine;
use tetronominoes::Tetronomino;

use crate::{
    my_engine::{vec2, Drawable, Vec2},
    tetronominoes::{Rotation, Variant},
};

fn main() {
    let mut engine = match MyEngine::init(30, 20, 2) {
        Ok(my_engine) => my_engine,
        Err(e) => panic!("We're having problems initialising the engine: {e}"),
    };

    let mut map = Map::empty(10, 10);
    let mut tet = Tetronomino::new(vec2::vec2(2, 2));

    loop {
        engine.clear_canvas();

        engine.set_pixels(map.get_pixels());
        // engine.set_pixels(tet.get_pixels());
        engine.set_shape(&tet);

        engine.draw();

        engine.process_input();

        tet.rotate_left();
    }
}
