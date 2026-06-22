use std::convert::Into;

use macroquad::prelude::{draw_rectangle, WHITE, BLACK};
use macroquad::window::miniquad::conf::Conf;

pub const PIXEL_SIZE: i32 = 16;
pub const SCREEN_WIDTH: i32 = 64;
pub const SCREEN_HEIGHT: i32 = 32;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("CHIP-8"),
        window_height: SCREEN_HEIGHT * PIXEL_SIZE,
        window_width: SCREEN_WIDTH * PIXEL_SIZE,
        window_resizable: false,

        ..Default::default()
    }
}

#[derive(Clone, Copy)]
enum Color {
    Black,
    White
}

impl Into<macroquad::color::Color> for Color {
    fn into(self) -> macroquad::color::Color {
        match self {
            Color::Black => BLACK,
            Color::White => WHITE
        }
    }
}

impl Color {
    fn invert(&mut self) {
        match self {
            Self::Black => *self = Color::White,
            Self::White => *self = Color::Black
        }
    }
}

type Row = [Color; SCREEN_WIDTH as usize];
pub struct Screen {
    pixels: [Row; SCREEN_HEIGHT as usize]
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [[Color::Black; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize]
        }
    }

    pub fn test() -> Self {
        let mut screen = Screen::new();

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if (x + y) % 2 == 0 {
                    screen.pixels[y as usize][x as usize] = Color::White;
                }
            }
        }
        screen
    }

    pub fn draw(&self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                draw_rectangle(
                    (x * PIXEL_SIZE) as f32,
                    (y * PIXEL_SIZE) as f32,
                    PIXEL_SIZE as f32,
                    PIXEL_SIZE as f32,
                    self.pixels[y as usize][x as usize].into()
                );
            }
        }
    }

    pub fn flip_pixel(&mut self, y: usize, x: usize) -> Result<(), String> {
        if y > SCREEN_HEIGHT as usize {
            return Err(format!("Cannot flip pixel at row {y}, screen only has {SCREEN_HEIGHT} rows !"));
        } else if x > SCREEN_WIDTH as usize {
            return Err(format!("Cannot flip pixel at column {x}, screen only has {SCREEN_WIDTH} columns !"));
        }
        self.pixels[y][x].invert();
        Ok(())
    }
}
