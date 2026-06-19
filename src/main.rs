use macroquad::prelude::*;
use macroquad::window::miniquad::conf::Conf;

const PIXEL_SIZE: i32 = 16;
const SCREEN_WIDTH: i32 = 64;
const SCREEN_HEIGHT: i32 = 32;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("CHIP-8"),
        window_height: SCREEN_HEIGHT * PIXEL_SIZE,
        window_width: SCREEN_WIDTH * PIXEL_SIZE,
        window_resizable: false,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                draw_rectangle((x * PIXEL_SIZE) as f32, (y * PIXEL_SIZE) as f32, PIXEL_SIZE as f32, PIXEL_SIZE as f32, if (x + y) % 2 == 0 { WHITE } else { BLACK });
            }
        }

        next_frame().await
    }
}
