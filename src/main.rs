mod score;
mod shapes;
mod grid;
mod game;
mod constants;
mod moves;
mod ai;

use game::Game;
use macroquad::{
    audio::{load_sound_from_bytes, play_sound, PlaySoundParams},
    window::next_frame, prelude::KeyCode,
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
- try WASM?
- try to train a self-learning agent
- teach bot about fall + shift
- add feature to collect stats on bot games
- find more efficient way of loading the music
 */

#[macroquad::main("Tetrus")]
async fn main() {
    let mut game = Game::new(true);
    // let music = load_sound_from_bytes(constants::MUSIC_BYTES).await.unwrap();
    // play_sound(
    //     music,
    //     PlaySoundParams {
    //         looped: true,
    //         volume: 0.5,
    //     },
    // );

    loop {
        game.update_game();
        next_frame().await;

        if macroquad::prelude::is_key_down(KeyCode::Q) {
            std::process::exit(0);
        }        
        // if macroquad::prelude::is_key_down(KeyCode::P) {
        //     thread::sleep(Duration::from_secs(10));
        // }
    }
}
