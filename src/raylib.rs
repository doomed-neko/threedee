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
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[link(name = "raylib", kind = "dylib")]
unsafe extern "C" {
    pub safe fn InitWindow(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        title: *const ::core::ffi::c_char,
    );
    pub safe fn ClearBackground(color: Color);
    pub safe fn BeginDrawing();
    pub safe fn EndDrawing();
    pub safe fn SetTargetFPS(fps: ::core::ffi::c_int);
    pub safe fn WindowShouldClose() -> bool;
    pub safe fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
    pub safe fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
}
