use std::time::SystemTime;

use crate::{grid::Grid, shapes::ShapePosition, score::Score, constants::USER_MOVE_DEBOUNCE_MS, moves::{Move, SimpleMove}};

pub struct Game {
    grid: Grid,
    current_shape: ShapePosition,
    last_drop_time: SystemTime,
    last_user_move_time: SystemTime,
    score: Score,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: Grid::new(),
            current_shape: ShapePosition::new(),
            last_drop_time: SystemTime::now(),
            last_user_move_time: SystemTime::now(),
            score: Score::new(),
        }
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

    fn should_debounce_block_drop(&self) -> bool {
        let ms_since_last_drop = SystemTime::now()
            .duration_since(self.last_drop_time)
            .unwrap();

        return ms_since_last_drop < self.score.get_block_drop_delay();
    }

    fn should_debounce_user_move(&self) -> bool {
        return SystemTime::now()
            .duration_since(self.last_user_move_time)
            .unwrap()
            .as_millis()
            < USER_MOVE_DEBOUNCE_MS;
    }

    fn perform_user_move(&mut self) {
        if self.should_debounce_user_move() {
            return;
        }

        match Move::from_key_press() {
            None => return,
            Some(Move::Simple(simple_move)) => {
                let new_pos = self.current_shape.moved_to(&simple_move);
                if self.is_valid_move(&new_pos) {
                    self.move_shape_to(new_pos)
                }
            }
            Some(Move::HardDrop) => {
                let mut did_hit_rock_bottom = false;
                while !did_hit_rock_bottom {
                    did_hit_rock_bottom = self.perform_block_drop();
                }
            }
        }

        self.last_user_move_time = SystemTime::now();
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
                println!("Game over!");
                std::process::exit(0);
            }
            self.current_shape = new_pos;
            self.add_shape_to_grid();
            return true;
        } else {
            self.move_shape_to(new_pos);
            return false;
        }
    }

    fn perform_block_drop_debounced(&mut self) {
        if self.should_debounce_block_drop() {
            return;
        }
        self.perform_block_drop();
        self.last_drop_time = SystemTime::now();
    }

    fn draw(&self) {
        self.grid.draw();
        self.score.draw();
    }

    pub fn update_game(&mut self) {
        // honor user-requested move if any
        self.perform_user_move();

        // move current block one step down
        self.perform_block_drop_debounced();

        self.draw();
    }
}
