use core::f32::consts::PI;

use threedee::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    raylib::{
        BeginDrawing, ClearBackground, Color, DrawText, EndDrawing, InitWindow, IsKeyDown,
        IsKeyPressed, KeyboardButton, SetTargetFPS, WindowShouldClose,
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
    let mut sides = 3;
    let mut radius = 0.5;
    let mut height = 0.5;
    let mut shape = shapes::regular_prism(sides, radius, height);
    while !WindowShouldClose() {
        // ROTATE
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

        // TRANSLATE Z
        if IsKeyDown(KeyboardButton::KeyZ) {
            dtz += 0.5 * dt;
        }
        if IsKeyDown(KeyboardButton::KeyX) {
            dtz -= 0.5 * dt;
            dtz = dtz.clamp(-1.4999999, f32::INFINITY);
        }

        // CONTROL SHAPE
        if IsKeyDown(KeyboardButton::KeyOne) {
            shape = shapes::cube();
        }
        if IsKeyDown(KeyboardButton::KeyTwo) {
            shape = shapes::pyramid();
        }
        if IsKeyDown(KeyboardButton::KeyThree) {
            shape = shapes::regular_prism(sides, radius, height);
        }

        // CONTROL SIDES
        if IsKeyPressed(KeyboardButton::KeyEqual) {
            sides += 1;
            shape = shapes::regular_prism(sides, radius, height);
        }
        if IsKeyPressed(KeyboardButton::KeyMinus) {
            sides -= 1;
            sides = sides.max(2);
            shape = shapes::regular_prism(sides, radius, height);
        }

        // CONTROL RADIUS
        if IsKeyDown(KeyboardButton::KeyLeftBracket) {
            radius -= 0.1 * dt;
            radius = radius.max(0.001);
            shape = shapes::regular_prism(sides, radius, height);
        }
        if IsKeyDown(KeyboardButton::KeyRightBracket) {
            radius += 0.1 * dt;
            shape = shapes::regular_prism(sides, radius, height);
        }

        // CONTROL HEIGHT
        if IsKeyDown(KeyboardButton::KeySemicolon) {
            height -= 0.1 * dt;
            height = height.max(0.001);
            shape = shapes::regular_prism(sides, radius, height);
        }
        if IsKeyDown(KeyboardButton::KeyApostrophe) {
            height += 0.1 * dt;
            shape = shapes::regular_prism(sides, radius, height);
        }

        // RESET
        if IsKeyDown(KeyboardButton::KeySpace) {
            dx = 0.;
            dy = 0.;
            dz = 0.;
            dtz = 0.;
            radius = 0.5;
            height = 0.5;
            sides = 4;
            shape = shapes::regular_prism(sides, radius, height);
        }
        BeginDrawing();
        ClearBackground(Color::BLACK);
        shape.draw(|vec| {
            vec.rotate_xz(dy)
                .rotate_xy(dz)
                .rotate_yz(dx)
                .translate_z(2. + dtz)
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
