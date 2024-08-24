use std::ops;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}
impl Vec2 {
    pub const fn new(x: i32, y: i32) -> Vec2 {
        Self { x, y }
    }
    pub const fn zero() -> Vec2 {
        const ZERO: Vec2 = Vec2 { x: 0, y: 0 };
        ZERO
    }
}
impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl ops::Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl ops::Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output::mul(rhs, self)
    }
}

pub const fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2::new(x, y)
}
