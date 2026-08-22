use raylib::prelude::*;
use threedee::{SCREEN_HEIGHT, SCREEN_WIDTH, shapes};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("3d bullshit")
        .build();

    let fps: f32 = 200.;
    rl.set_target_fps(fps as u32);
    let dt = 1. / fps;
    let mut dr = 0.;
    while !rl.window_should_close() {
        dr += PI as f32 * dt;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        shapes::pyramid().draw(&mut d, |vec| {
            vec.rotate_xz(dr).translate_z(2.).project2d().screen_cords()
        });
    }
}
