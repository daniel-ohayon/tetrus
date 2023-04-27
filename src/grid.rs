use macroquad::{
    prelude::BLUE,
    shapes::{draw_line, draw_rectangle},
};

use crate::{
    constants::{BLOCK_SIZE, CELL_BORDER, GRID_HEIGHT, GRID_WIDTH},
    shapes::SHAPE_COLORS,
};

pub const EMPTY_CELL: i32 = -1;

#[derive(Clone)]
pub struct Grid {
    // a 2D array where each cell represents a cell on the Tetris grid
    pub grid: [[i32; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: [[EMPTY_CELL; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
        }
    }

    pub fn can_set_pixels(
        &self,
        pixels_to_set: &[(i16, i16)],
        pixels_to_disable: &[(i16, i16)],
    ) -> bool {
        for (i, j) in pixels_to_set {
            if *i < 0 || *j < 0 || *i >= GRID_HEIGHT || *j >= GRID_WIDTH {
                // cell is out of bounds
                return false;
            }
            if pixels_to_disable.contains(&(*i, *j)) {
                // if the cell is currently occupied by the shape we're going to move,
                // ignore it
                continue;
            }
            if self.grid[*i as usize][*j as usize] != EMPTY_CELL {
                // cell is occupied
                return false;
            }
        }
        return true;
    }

    pub fn set_pixels(&mut self, pixels: &[(i16, i16)], color: i32) {
        for (i, j) in pixels {
            self.grid[*i as usize][*j as usize] = color;
        }
    }

    pub fn unset_pixels(&mut self, pixels: &[(i16, i16)]) {
        self.set_pixels(pixels, EMPTY_CELL);
    }

    pub fn clear_completed_rows(&mut self) -> i32 {
        // We use naive gravity and support split line clears
        // See https://tetris.wiki/Line_clear
        let mut n_cleared = 0;
        for i in 0..(GRID_HEIGHT as usize) {
            if self.grid[i].iter().all(|&x| x != EMPTY_CELL) {
                self.shift_rows_down(i);
                n_cleared += 1;
            }
        }
        return n_cleared;
    }

    fn shift_rows_down(&mut self, start_index: usize) {
        let mut i = start_index;
        while i > 0 {
            self.grid[i] = self.grid[i - 1];
            i -= 1;
        }
        self.grid[0] = [EMPTY_CELL; GRID_WIDTH as usize];
    }

    pub fn draw(&self) {
        for i in 0..GRID_HEIGHT {
            // stops at HEIGHT-1
            for j in 0..GRID_WIDTH {
                // stops at WIDTH-1
                let pixel_color = self.grid[i as usize][j as usize];
                if pixel_color != EMPTY_CELL {
                    draw_rectangle(
                        (j * BLOCK_SIZE) as f32 + CELL_BORDER,
                        (i * BLOCK_SIZE) as f32 + CELL_BORDER,
                        BLOCK_SIZE as f32 - CELL_BORDER,
                        BLOCK_SIZE as f32 - CELL_BORDER,
                        SHAPE_COLORS[pixel_color as usize],
                    );
                }
            }
        }

        let height_px = (BLOCK_SIZE * GRID_HEIGHT) as f32;
        let width_px = (GRID_WIDTH * BLOCK_SIZE) as f32;
        draw_line(width_px, 0f32, width_px, height_px, 1f32, BLUE);

        draw_line(0f32, height_px, width_px, height_px, 1f32, BLUE);
    }
}
