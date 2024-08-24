use std::slice::SliceIndex;

use crate::my_engine::{shape::Shape, vec2::vec2, Drawable, MyEngine, Pixel, Vec2};
use console_engine::Color;

pub struct Tetronomino {
    variant: Variant,
    rotation: Rotation,
    origin: Vec2,
    color: Color,
}
impl Shape for Tetronomino {
    fn get_origin(&self) -> Vec2 {
        self.origin
    }

    fn get_segments(&self) -> &'static [Vec2; 4] {
        let v: usize = self.variant.into();
        let r: usize = self.rotation.into();
        &TETRONOMINOES[v][r]
    }
}
impl Tetronomino {
    pub fn new(variant: Variant, origin: Vec2) -> Self {
        Self {
            variant,
            rotation: Rotation::Default,
            origin,
            color: Color::Blue,
        }
    }
    pub fn rotate_right(&mut self) {
        self.rotation = match self.rotation {
            Rotation::Default => Rotation::Quarter,
            Rotation::Quarter => Rotation::Half,
            Rotation::Half => Rotation::ThreeQuarters,
            Rotation::ThreeQuarters => Rotation::Default,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Rotation {
    Default,
    Quarter,
    Half,
    ThreeQuarters,
}
impl From<Rotation> for usize {
    fn from(value: Rotation) -> Self {
        value as usize
    }
}

#[derive(Clone, Copy)]
pub enum Variant {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}
impl From<Variant> for usize {
    fn from(value: Variant) -> Self {
        value as usize
    }
}

type Variants = [Rotations; 7];
type Rotations = [Segments; 4];
type Segments = [PositionRelativeToOrigin; 4];
type PositionRelativeToOrigin = Vec2;

// TODO: fill the other shapes
const TETRONOMINOES: Variants = [
    // I
    [
        [vec2(0, -2), vec2(0, -1), vec2(0, 0), vec2(0, 1)],
        [vec2(-1, 0), vec2(0, 0), vec2(1, 0), vec2(2, 0)],
        [vec2(0, -1), vec2(0, 0), vec2(0, 1), vec2(0, 2)],
        [vec2(-2, 0), vec2(-1, 0), vec2(0, 0), vec2(1, 0)],
    ],
    // O
    [
        [vec2(0, 0), vec2(-1, 0), vec2(-1, -1), vec2(0, -1)],
        [vec2(1, 0), vec2(0, 0), vec2(0, -1), vec2(1, -1)],
        [vec2(0, 0), vec2(1, 0), vec2(0, 1), vec2(1, 1)],
        [vec2(-1, 0), vec2(0, 0), vec2(-1, 1), vec2(0, 1)],
    ],
    // T
    [
        [vec2(-1, 0), vec2(0, 0), vec2(1, 0), vec2(0, -1)],
        [vec2(0, -1), vec2(0, 0), vec2(0, 1), vec2(1, 0)],
        [vec2(-1, 0), vec2(0, 0), vec2(1, 0), vec2(0, 1)],
        [vec2(0, -1), vec2(0, 0), vec2(0, 1), vec2(-1, 0)],
    ],
    // J
    [
        [vec2(-1, 1), vec2(0, -1), vec2(0, 0), vec2(0, 1)],
        [vec2(-1, -1), vec2(-1, 0), vec2(0, 0), vec2(1, 0)],
        [vec2(0, -1), vec2(0, 0), vec2(0, 1), vec2(1, -1)],
        [vec2(-1, 0), vec2(0, 0), vec2(1, 0), vec2(1, 1)],
    ],
    // L
    [
        [vec2(0, -1), vec2(0, 0), vec2(0, 1), vec2(1, 1)],
        [vec2(-1, 0), vec2(0, 0), vec2(1, 0), vec2(-1, 1)],
        [vec2(-1, -1), vec2(0, -1), vec2(0, 0), vec2(0, 1)],
        [vec2(-1, 0), vec2(0, 0), vec2(1, 0), vec2(1, -1)],
    ],
    // S
    [
        [vec2(-1, 0), vec2(0, 0), vec2(0, -1), vec2(1, -1)],
        [vec2(0, -1), vec2(0, 0), vec2(1, 0), vec2(1, 1)],
        [vec2(0, 1), vec2(-1, 1), vec2(0, 0), vec2(1, 0)],
        [vec2(0, 0), vec2(-1, -1), vec2(-1, 0), vec2(0, 1)],
    ],
    // Z
    [
        [vec2(1, 0), vec2(0, -1), vec2(-1, -1), vec2(0, 0)],
        [vec2(0, 1), vec2(1, 0), vec2(1, -1), vec2(0, 0)],
        [vec2(1, 1), vec2(0, 0), vec2(-1, 0), vec2(0, 1)],
        [vec2(-1, 1), vec2(0, 0), vec2(0, -1), vec2(-1, 0)],
    ],
];
