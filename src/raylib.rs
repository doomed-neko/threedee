use crate::vec::Vector2;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub const GREEN: Color = Color::new(0, 128, 0, 255);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[link(name = "raylib", kind = "dylib")]
unsafe extern "C" {
    pub safe fn InitWindow(width: i32, height: i32, title: *const ::core::ffi::c_char);
    pub safe fn DrawText(
        text: *const u8,
        posX: i32,
        posY: i32,
        fontSize: ::core::ffi::c_int,
        color: Color,
    ); // Draw text (using default font)
    pub safe fn ClearBackground(color: Color);
    pub safe fn BeginDrawing();
    pub safe fn EndDrawing();
    pub safe fn SetTargetFPS(fps: ::core::ffi::c_int);
    pub safe fn WindowShouldClose() -> bool;
    pub safe fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
    pub safe fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
    pub safe fn IsKeyDown(key: KeyboardButton) -> bool;
}

#[repr(C)]
pub enum KeyboardButton {
    KeySpace = 32, // Key: Space
    KeyOne = 49,   // Key: 1
    KeyTwo = 50,   // Key: 2
    KeyA = 65,     // Key: A | a
    KeyD = 68,     // Key: D | d
    KeyE = 69,     // Key: E | e
    KeyQ = 81,     // Key: Q | q
    KeyS = 83,     // Key: S | s
    KeyW = 87,     // Key: W | w
    KeyX = 88,     // Key: X | x
    KeyZ = 90,     // Key: Z | z
}
