use macroquad::prelude::*;

mod screen;
use screen::*;

const AUDIO_PATH: &str = "mixkit-video-game-lock-2851.wav";

#[macroquad::main(window_conf)]
async fn main() {
    let sound = macroquad::audio::load_sound(AUDIO_PATH).await;
    if let Err(err) = sound {
        eprintln!("Impossible d'ouvrir le fichier son '{AUDIO_PATH}': '{err}'");
        return;
    }
    let sound = sound.unwrap();
    let mut screen = screen::Screen::test();

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Space) {
            macroquad::audio::play_sound_once(&sound);
            if let Err(err) = screen.flip_pixel(0, 0) {
                eprintln!("Error while flipping pixel: {err}");
                break;
            }
        }

        screen.draw();

        next_frame().await
    }
}
