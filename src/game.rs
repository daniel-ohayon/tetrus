use std::time::Duration;

use macroquad::{
    prelude::{KeyCode, WHITE},
    text::{draw_text, measure_text},
    window::{next_frame, screen_height, screen_width},
};

use crate::{
    ai::TetrisBot,
    events::{Event, EventLog},
    grid::Grid,
    moves::{Move, SimpleMove},
    music::MusicPlayer,
    score::Score,
    shapes::ShapePosition,
};

pub struct Game {
    grid: Grid,
    // info about the tetromino that the user currently controls
    current_shape: ShapePosition,

    pub score: Score,
    bot: Option<TetrisBot>,

    music_player: MusicPlayer,
    event_log: EventLog,
    clock_speedup_rate: u32,
    no_screen: bool,
}

impl Game {
    const USER_MOVE_DEBOUNCE: Duration = Duration::from_millis(100);

    pub fn new(use_ai: bool, speedup_rate: u32, no_screen: bool) -> Self {
        let grid = Grid::new();
        let current_shape = ShapePosition::new();
        let bot = use_ai.then(|| {
            let mut _bot = TetrisBot::new();
            _bot.update_policy(&grid, &current_shape);
            return _bot;
        });

        return Game {
            grid,
            current_shape,
            score: Score::new(),
            bot,
            music_player: MusicPlayer::new(!no_screen),
            event_log: EventLog::new(),
            clock_speedup_rate: speedup_rate,
            no_screen,
        };
    }

    fn get_shape_pixels(&self) -> [(i16, i16); 4] {
        return self.current_shape.get_pixels();
    }

    fn is_valid_move(&self, new_pos: &ShapePosition) -> bool {
        // check if the current shape can be moved from its current position
        // to its desired next position
        let new_pixels = new_pos.get_pixels();
        return self
            .grid
            .can_set_pixels(&new_pixels, &self.get_shape_pixels());
    }

    fn is_valid_add(&self, new_pos: &ShapePosition) -> bool {
        // check if a new piece can be added to the board (if not, game over)
        return self.grid.can_set_pixels(&new_pos.get_pixels(), &[]);
    }

    fn clear_shape_from_grid(&mut self) {
        self.grid.unset_pixels(&self.get_shape_pixels());
    }

    fn add_shape_to_grid(&mut self) {
        self.grid.set_pixels(
            &self.get_shape_pixels(),
            self.current_shape.color_index as i32,
        );
    }

    fn move_shape_to(&mut self, new_pos: ShapePosition) {
        self.clear_shape_from_grid();
        self.current_shape = new_pos;
        self.add_shape_to_grid();
    }

    fn perform_user_move(&mut self) {
        // In no-screen mode, just perform all the bot-requested moves
        // and ignore gravity
        if self.no_screen {
            while let Some(user_move) = self.get_move_from_human_or_bot() {
                self.apply_move(&user_move);
            }
            return;
        }

        if !self.event_log.elapsed_since(
            Event::UserMove,
            Self::USER_MOVE_DEBOUNCE / self.clock_speedup_rate,
        ) {
            return;
        }

        if let Some(user_move) = self.get_move_from_human_or_bot() {
            self.apply_move(&user_move);
            self.event_log.register_event(Event::UserMove);
        }
    }

    fn get_move_from_human_or_bot(&mut self) -> Option<Move> {
        if let Some(bot) = &mut self.bot {
            return bot.pop_next_move();
        } else {
            return Move::from_key_press();
        }
    }

    fn apply_move(&mut self, move_: &Move) {
        match move_ {
            Move::Simple(simple_move) => {
                let new_pos = self.current_shape.moved_to(&simple_move);
                if self.is_valid_move(&new_pos) {
                    self.move_shape_to(new_pos);
                }
            }
            Move::HardDrop => {
                let mut did_hit_rock_bottom = false;
                while !did_hit_rock_bottom {
                    did_hit_rock_bottom = self.perform_block_drop();
                }
            }
        }
    }

    fn game_over(&mut self) {
        self.event_log.register_event(Event::GameOver);
        self.music_player.play_game_over();
    }

    fn draw_game_over_screen(&self) {
        let message = "Game over";
        let font_size = 60;
        let text_dims = measure_text(message, None, font_size, 1.);
        draw_text(
            message,
            (screen_width() - text_dims.width) / 2.,
            (screen_height() - text_dims.height) / 2.,
            font_size as f32,
            WHITE,
        );
    }

    fn perform_block_drop(&mut self) -> bool {
        // try to move the current piece one cell down
        let mut new_pos = self.current_shape.moved_to(&SimpleMove::Down);
        if !self.is_valid_move(&new_pos) {
            // we can't move down any further: issue a new piece
            // first, check if any line got cleared
            let n_cleared = self.grid.clear_completed_rows();
            self.score.update(n_cleared);

            // then drop the next piece
            new_pos = ShapePosition::new();
            if !self.is_valid_add(&new_pos) {
                // show the piece overlap with existing pieces
                // for dramatic effect
                self.current_shape = new_pos;
                self.add_shape_to_grid();
                self.game_over();
                return true;
            }
            self.current_shape = new_pos;
            self.add_shape_to_grid();

            if let Some(bot) = &mut self.bot {
                bot.update_policy(&self.grid, &self.current_shape)
            }

            return true;
        } else {
            self.move_shape_to(new_pos);
            return false;
        }
    }

    fn perform_block_drop_debounced(&mut self) {
        if !self.no_screen && !self.event_log.elapsed_since(
            Event::GravityDrop,
            self.score.get_block_drop_delay() / self.clock_speedup_rate,
        ) {
            return;
        }
        self.perform_block_drop();
        self.event_log.register_event(Event::GravityDrop);
    }

    fn draw(&self) {
        if self.no_screen {
            return;
        }
        self.grid.draw();
        self.score.draw();
    }

    pub async fn play(&mut self) {
        loop {
            // main event loop
            if !self.update_game() {
                break;
            }
            if !self.no_screen {
                next_frame().await;
            }

            if macroquad::prelude::is_key_down(KeyCode::Q) {
                break;
            }
        }
    }

    // returns a bool indicating whether the game should keep going
    fn update_game(&mut self) -> bool {
        if self.event_log.did_happen(Event::GameOver) {
            if self.no_screen {
                return false;
            }
            self.draw_game_over_screen();

            if self
                .event_log
                .elapsed_since(Event::GameOver, Duration::from_secs(3))
            {
                return false;
            }
            return true;
        }

        // honor user-requested move if any
        self.perform_user_move();

        // move current block one step down
        self.perform_block_drop_debounced();

        self.draw();
        return true;
    }
}
