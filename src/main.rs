mod score;
mod shapes;
mod grid;
mod game;
mod constants;
mod moves;

use game::Game;
use macroquad::{
    audio::{load_sound_from_bytes, play_sound, PlaySoundParams},
    window::next_frame,
};

// macroquad docs:
// https://macroquad.rs/examples/
//
// ----> x
// |
// v y

// https://mathworld.wolfram.com/Tetromino.html



/*
TODO
- add sound effect when clearing line
- add GAME OVER effect
- implement instant drop
- scoring for instant drop
- refactor into multiple files?
- speed up fall when clearing level
- try WASM?
- try to train an AI
 */

#[macroquad::main("Tetrus")]
async fn main() {
    let mut game = Game::new();
    let music = load_sound_from_bytes(constants::MUSIC_BYTES).await.unwrap();
    play_sound(
        music,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );

    loop {
        game.update_game();
        next_frame().await;
        // if macroquad::prelude::is_key_down(KeyCode::P) {
        //     thread::sleep(Duration::from_secs(10));
        // }
    }
}
