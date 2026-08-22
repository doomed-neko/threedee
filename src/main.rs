use core::f32::consts::PI;

use threedee::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    raylib::{
        BeginDrawing, ClearBackground, Color, DrawText, EndDrawing, InitWindow, IsKeyDown,
        KeyboardButton, SetTargetFPS, WindowShouldClose,
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
    let mut dtz = 0.;
    let mut dy = 0.;
    let mut dz = 0.;
    let mut dx = 0.;
    let mut shape = shapes::pyramid();
    while !WindowShouldClose() {
        if IsKeyDown(KeyboardButton::KeyD) {
            dy += PI * dt;
        }
        if IsKeyDown(KeyboardButton::KeyA) {
            dy -= PI * dt;
        }
        if IsKeyDown(KeyboardButton::KeyW) {
            dx += PI * dt;
        }
        if IsKeyDown(KeyboardButton::KeyS) {
            dx -= PI * dt;
        }
        if IsKeyDown(KeyboardButton::KeyQ) {
            dz += PI * dt;
        }
        if IsKeyDown(KeyboardButton::KeyE) {
            dz -= PI * dt;
        }
        if IsKeyDown(KeyboardButton::KeyZ) {
            dtz += 0.5 * dt;
        }
        if IsKeyDown(KeyboardButton::KeyX) {
            dtz -= 0.5 * dt;
            dtz = dtz.clamp(-1.4999999, f32::INFINITY);
        }
        if IsKeyDown(KeyboardButton::KeyOne) {
            shape = shapes::cube();
        }
        if IsKeyDown(KeyboardButton::KeyTwo) {
            shape = shapes::pyramid();
        }
        if IsKeyDown(KeyboardButton::KeySpace) {
            dx = 0.;
            dy = 0.;
            dz = 0.;
            dtz = 0.;
        }
        BeginDrawing();
        ClearBackground(Color::BLACK);
        shape.draw(|vec| {
            vec.rotate_xz(dy)
                .rotate_xy(dz)
                .rotate_yz(dx)
                .translate_z(2. + dtz)
                .project2d()
                .screen_cords()
        });
        DrawText(
            format!("x rotation: {dx}\ny rotation: {dy}\nz rotation: {dz}\nz translation: {dtz}\0")
                .as_ptr(),
            20,
            20,
            20,
            Color::WHITE,
        );
        EndDrawing();
    }
}
