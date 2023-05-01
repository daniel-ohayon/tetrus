use std::time::Duration;

use macroquad::{text::draw_text, prelude::WHITE};

use crate::constants::{GRID_WIDTH, BLOCK_SIZE};

pub struct Score {
    pub points: i32,
    pub level: i32,
    pub total_lines_cleared: i32,
}

impl Score {
    const FONT_SIZE: f32 = 40.;
    const LEFT_PADDING: f32 = GRID_WIDTH as f32 * BLOCK_SIZE as f32 + 100f32;
    const TOP_OFFSET: f32 = 200.;
    const MARGIN_BETWEEN_STATS: f32 = 40.;

    pub fn new() -> Self {
        Score {
            points: 0,
            level: 0,
            total_lines_cleared: 0,
        }
    }

    pub fn update(&mut self, n_lines_cleared: i32) -> bool {
        let mut base = 0;
        if n_lines_cleared == 0 {
            return false;
        }
        match n_lines_cleared {
            1 => base = 40,
            2 => base = 100,
            3 => base = 300,
            4 => base = 1200,
            _ => (),
        }
        self.points += base * (self.level + 1);

        self.total_lines_cleared += n_lines_cleared;
        let curr_level = self.total_lines_cleared / 10;
        let did_level_up = curr_level > self.level;
        self.level = curr_level;
        return did_level_up;
    }

    fn draw_text_at(&self, text: &str, position: u8) {
        draw_text(
            text,
            Score::LEFT_PADDING,
            Score::TOP_OFFSET + position as f32 * Score::MARGIN_BETWEEN_STATS,
            Score::FONT_SIZE,
            WHITE,
        );
    }

    pub fn draw(&self) {
        self.draw_text_at(&format!("Score: {}", self.points), 0);
        self.draw_text_at(&format!("Level: {}", self.level), 1);
        self.draw_text_at(&format!("Lines cleared: {}", self.total_lines_cleared), 2);
    }

    pub fn get_block_drop_delay(&self) -> Duration {
        // from https://tetris.wiki/Marathon
        return Duration::from_secs_f64(f64::powi(
            0.8 - (self.level as f64 - 1.) * 0.007,
            self.level - 1,
        ));
    }
}
