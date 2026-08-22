use core::f32::consts::PI;

use threedee::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    raylib::{
        BeginDrawing, ClearBackground, Color, DrawText, EndDrawing, InitWindow, IsKeyDown,
        SetTargetFPS, WindowShouldClose,
    },
    shapes,
};

pub fn main() {
    InitWindow(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "3d bullshit".as_ptr() as *const i8,
    );

    let fps: f32 = 200.;
    SetTargetFPS(fps as i32);
    let dt = 1. / fps;
    let mut dy = 0.;
    let mut dz = 0.;
    let mut dx = 0.;
    let mut shape = shapes::pyramid();
    while !WindowShouldClose() {
        if IsKeyDown(threedee::raylib::KeyboardButton::KeyD) {
            dy += PI * dt;
        }
        if IsKeyDown(threedee::raylib::KeyboardButton::KeyA) {
            dy -= PI * dt;
        }
        if IsKeyDown(threedee::raylib::KeyboardButton::KeyW) {
            dx += PI * dt;
        }
        if IsKeyDown(threedee::raylib::KeyboardButton::KeyS) {
            dx -= PI * dt;
        }
        if IsKeyDown(threedee::raylib::KeyboardButton::KeyQ) {
            dz += PI * dt;
        }
        if IsKeyDown(threedee::raylib::KeyboardButton::KeyE) {
            dz -= PI * dt;
        }
        if IsKeyDown(threedee::raylib::KeyboardButton::KeyOne) {
            shape = shapes::cube();
        }
        if IsKeyDown(threedee::raylib::KeyboardButton::KeyTwo) {
            shape = shapes::pyramid();
        }
        BeginDrawing();
        ClearBackground(Color::BLACK);
        shape.draw(|vec| {
            vec.rotate_xz(dy)
                .rotate_xy(dz)
                .rotate_yz(dx)
                .translate_z(2.)
                .project2d()
                .screen_cords()
        });
        DrawText(
            format!("x rotation: {dx}\ny rotation: {dy}\nz rotation: {dz}\0").as_ptr(),
            20,
            20,
            20,
            Color::WHITE,
        );
        EndDrawing();
    }
}
