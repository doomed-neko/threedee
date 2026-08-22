#![no_std]
#![no_main]
use core::f32::consts::PI;

use raylib_sys::{
    BeginDrawing, ClearBackground, Color, EndDrawing, SetTargetFPS, WindowShouldClose,
};
use threedee::{SCREEN_HEIGHT, SCREEN_WIDTH, shapes};

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    unsafe {
        raylib_sys::InitWindow(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "3d bullshit".as_ptr() as *const i8,
        );

        let fps: f32 = 200.;
        SetTargetFPS(fps as i32);
        let dt = 1. / fps;
        let mut dr = 0.;
        while !WindowShouldClose() {
            dr += PI * dt;
            BeginDrawing();
            ClearBackground(Color::BLACK);
            shapes::pyramid()
                .draw(|vec| vec.rotate_xz(dr).translate_z(2.).project2d().screen_cords());
            EndDrawing();
        }
    }
}
