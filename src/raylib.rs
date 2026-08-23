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
    pub const LIGHTGRAY: Color = Color::new(200, 200, 200, 255); // Light Gray
    pub const GRAY: Color = Color::new(130, 130, 130, 255); // Gray
    pub const DARKGRAY: Color = Color::new(80, 80, 80, 255); // Dark Gray
    pub const YELLOW: Color = Color::new(253, 249, 0, 255); // Yellow
    pub const GOLD: Color = Color::new(255, 203, 0, 255); // Gold
    pub const ORANGE: Color = Color::new(255, 161, 0, 255); // Orange
    pub const PINK: Color = Color::new(255, 109, 194, 255); // Pink
    pub const RED: Color = Color::new(230, 41, 55, 255); // Red
    pub const MAROON: Color = Color::new(190, 33, 55, 255); // Maroon
    pub const GREEN: Color = Color::new(0, 228, 48, 255); // Green
    pub const LIME: Color = Color::new(0, 158, 47, 255); // Lime
    pub const DARKGREEN: Color = Color::new(0, 117, 44, 255); // Dark Green
    pub const SKYBLUE: Color = Color::new(102, 191, 255, 255); // Sky Blue
    pub const BLUE: Color = Color::new(0, 121, 241, 255); // Blue
    pub const DARKBLUE: Color = Color::new(0, 82, 172, 255); // Dark Blue
    pub const PURPLE: Color = Color::new(200, 122, 255, 255); // Purple
    pub const VIOLET: Color = Color::new(135, 60, 190, 255); // Violet
    pub const DARKPURPLE: Color = Color::new(112, 31, 126, 255); // Dark Purple
    pub const BEIGE: Color = Color::new(211, 176, 131, 255); // Beige
    pub const BROWN: Color = Color::new(127, 106, 79, 255); // Brown
    pub const DARKBROWN: Color = Color::new(76, 63, 47, 255); // Dark Brown

    pub const WHITE: Color = Color::new(255, 255, 255, 255); // White
    pub const BLACK: Color = Color::new(0, 0, 0, 255); // Black
    pub const BLANK: Color = Color::new(0, 0, 0, 0); // Blank (Transparent)
    pub const MAGENTA: Color = Color::new(255, 0, 255, 255); // Magenta
    pub const RAYWHITE: Color = Color::new(245, 245, 245, 255); // My own White (raylib logo)
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
    pub safe fn IsKeyPressed(key: KeyboardButton) -> bool;
    pub safe fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color); // Draw a color-filled triangle (vertex in counter-clockwise order!)
}

#[repr(C)]
pub enum KeyboardButton {
    KeySpace = 32,        // Key: Space
    KeyApostrophe = 39,   // Key: '
    KeyMinus = 45,        // Key: -
    KeyOne = 49,          // Key: 1
    KeyTwo = 50,          // Key: 2
    KeyThree = 51,        // Key: 3
    KeySemicolon = 59,    // Key: ;
    KeyEqual = 61,        // Key: =
    KeyA = 65,            // Key: A | a
    KeyD = 68,            // Key: D | d
    KeyE = 69,            // Key: E | e
    KeyQ = 81,            // Key: Q | q
    KeyS = 83,            // Key: S | s
    KeyW = 87,            // Key: W | w
    KeyX = 88,            // Key: X | x
    KeyZ = 90,            // Key: Z | z
    KeyLeftBracket = 91,  // Key: [
    KeyRightBracket = 93, // Key: ]
}
