use std::ops::{Add, Mul, Neg, Sub};

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    raylib::{Color, DrawRectangleV},
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const ZEROED: Self = Self::new(0., 0., 0.);
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
    pub fn rotate_xy(self, angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
            z: self.z,
        }
    }
    pub fn rotate_yz(self, angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self {
            x: self.x,
            y: self.z * s + self.y * c,
            z: self.z * c - self.y * s,
        }
    }
    pub fn cross_product(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x - rhs.z,
            z: self.x * rhs.y - self.y * rhs.z,
        }
    }
    pub fn dot_product(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
    pub fn to_vec(self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
    pub fn draw(self) {
        DrawRectangleV(
            self.to_vec(),
            Vector2::new(Self::DRAW_SIZE, Self::DRAW_SIZE),
            Color::GREEN,
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
