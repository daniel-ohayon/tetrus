use macroquad::prelude::{YELLOW, ORANGE, BLUE, PURPLE, GREEN, RED, WHITE};
use rand::Rng;

use crate::{constants::GRID_WIDTH, moves::SimpleMove};


pub const SHAPE_COLORS: [macroquad::prelude::Color; 7] =
    [YELLOW, ORANGE, BLUE, PURPLE, GREEN, RED, WHITE];

pub fn get_shapes() -> [Vec<[(i16, i16); 4]>; 6] {
    return [
        // square
        vec![[(0, 0), (0, 1), (1, 1), (1, 0)]],
        // straight
        vec![
            [(0, 0), (0, 1), (0, 2), (0, 3)],
            [(-1, 1), (0, 1), (1, 1), (2, 1)],
        ],
        // T-shape
        vec![
            [(0, 0), (0, 1), (0, 2), (1, 1)],
            [(0, 0), (0, 1), (-1, 1), (1, 1)],
            [(0, 0), (0, 1), (0, 2), (-1, 1)],
            [(0, 1), (-1, 1), (1, 1), (0, 2)],
        ],
        // Skew 1
        vec![
            [(0, 0), (0, 1), (1, 1), (1, 2)],
            [(0, 1), (0, 2), (-1, 2), (1, 1)],
        ],
        // Skew 2,
        vec![
            [(0, 1), (0, 2), (1, 1), (1, 0)],
            [(0, 1), (-1, 1), (0, 2), (1, 2)],
        ],
        // L shape 1
        vec![
            [(0, 0), (0, 1), (0, 2), (1, 0)],
            [(-1, 0), (-1, 1), (0, 1), (1, 1)],
            [(0, 0), (0, 1), (0, 2), (-1, 2)],
            [(-1, 1), (0, 1), (1, 1), (1, 2)],
        ],
    ];
}

#[derive(Clone, Copy, Debug)]
pub struct ShapePosition {
    pos: (i16, i16),
    shape_index: usize,
    pub rotation_index: usize,
    pub color_index: usize,
}

impl ShapePosition {
    pub fn new() -> Self {
        ShapePosition {
            pos: (0, (GRID_WIDTH / 2) as i16), // topleft pixel index ("offset" ?)
            shape_index: rand::thread_rng().gen_range(0..6),
            rotation_index: 0,
            color_index: rand::thread_rng().gen_range(0..SHAPE_COLORS.len()),
        }
    }

    pub fn n_rotations(&self) -> usize {
        return get_shapes()[self.shape_index].len();
    }

    pub fn moved_to(&self, move_: &SimpleMove) -> Self {
        let mut new_pos = self.clone();
        match move_ {
            SimpleMove::Left => new_pos.pos.1 -= 1,
            SimpleMove::Right => new_pos.pos.1 += 1,
            SimpleMove::Down => new_pos.pos.0 += 1,
            SimpleMove::Rotate => {
                let n_rotations = get_shapes()[new_pos.shape_index].len();
                new_pos.rotation_index = (new_pos.rotation_index + 1) % n_rotations
            }
        }
        return new_pos;
    }

    pub fn n_moves(&self, horizontal_shift: i16, n_rotations: usize, vertical_shift: u16) -> Self {
        let mut new_pos = self.clone();
        new_pos.rotation_index += n_rotations;
        new_pos.pos.1 += horizontal_shift;
        new_pos.pos.0 += vertical_shift as i16;
        return new_pos;
    }

    pub fn get_pixels(&self) -> [(i16, i16); 4] {
        let shape_pixels = get_shapes()[self.shape_index][self.rotation_index];
        let mut result = [(0, 0); 4];
        let mut idx = 0;
        for pixel in shape_pixels {
            let i = pixel.0 + self.pos.0;
            let j = pixel.1 + self.pos.1;
            result[idx] = (i, j);
            idx += 1;
        }
        return result;
    }
}