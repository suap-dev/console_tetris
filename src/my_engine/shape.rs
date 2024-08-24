use crate::my_engine::Vec2;

pub(crate) trait Shape {
    fn get_origin(&self) -> Vec2;
    fn get_segments(&self) -> &'static [Vec2; 4];
}
