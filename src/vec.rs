use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use raylib::{
    drawing::RaylibDraw,
    drawing::RaylibDrawHandle,
    ffi::{Color, Vector2},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl V2 {
    const DRAW_SIZE: f32 = 10.;
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub const fn translate_z(self, z: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + z,
        }
    }
    pub const fn project2d(self) -> Self {
        Self {
            x: self.x / self.z,
            y: self.y / self.z,
            z: self.z,
        }
    }
    pub const fn screen_cords(self) -> Self {
        let size_offset = Self::DRAW_SIZE / 2.;
        let screen_x = (self.x + 1.) / 2. * SCREEN_WIDTH as f32;
        let screen_y = (1. - (self.y + 1.) / 2.) * SCREEN_HEIGHT as f32;
        Self {
            x: screen_x - size_offset,
            y: screen_y - size_offset,
            z: self.z,
        }
    }
    pub fn rotate_xz(self, angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self {
            x: self.x * c - self.z * s,
            y: self.y,
            z: self.x * s + self.z * c,
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(self, (Self::DRAW_SIZE, Self::DRAW_SIZE), Color::GREEN)
    }
}
impl From<&V2> for Vector2 {
    fn from(val: &V2) -> Self {
        Vector2 { x: val.x, y: val.y }
    }
}
impl From<V2> for Vector2 {
    fn from(val: V2) -> Self {
        Vector2 { x: val.x, y: val.y }
    }
}
