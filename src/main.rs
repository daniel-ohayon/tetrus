mod ai;
mod constants;
mod events;
mod game;
mod grid;
mod moves;
mod score;
mod shapes;
mod music;
mod stats;

use game::Game;
use macroquad::window::{screen_height, screen_width};

use rustop::opts;

use crate::score::Score;

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
- try WASM?
- try to train a self-learning agent
- teach bot about fall + shift
- mode with no rendering and no time for bot training/testing
- find more efficient way of loading the music
 */

#[macroquad::main("Tetrus")]
async fn main() {
    let (args, _rest) = opts! {
        synopsis "A Tetris game implemented in Rust.";
        opt autoplay:bool, desc:"Auto-play by AI";
        opt n_games: usize=1, desc:"Number of games to play";
        opt speedup: Option<u32>, desc:"Speedup rate of the game";
        opt no_screen: bool, desc:"Do not display the game on screen (for AI testing)";
    }
    .parse_or_exit();

    println!("Width: {}, Height: {}", screen_width(), screen_height());

    let mut scores: Vec<Score> = Vec::new();
    let speedup = args.speedup.unwrap_or(if args.autoplay {10} else {1});
    
    for i in 0..args.n_games {
        println!("Game {}/{}", i+1, args.n_games);
        let mut game = Game::new(args.autoplay, speedup, args.no_screen);
        game.play().await;
        scores.push(game.score);
    }

    println!("Summary stats over {} games:", args.n_games);
    println!(
        "Average score: {}",
        stats::avg(&scores.iter().map(|s| s.points).collect())
    );
    println!(
        "Average number of lines cleared: {}",
        stats::avg(&scores.iter().map(|s| s.total_lines_cleared).collect())
    );
    println!(
        "Average level attained: {}",
        stats::avg(&scores.iter().map(|s| s.level).collect())
    );
}
