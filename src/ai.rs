use std::iter;

use crate::{
    constants::{GRID_HEIGHT, GRID_WIDTH},
    grid::{Grid, EMPTY_CELL},
    moves::{Move, SimpleMove},
    shapes::ShapePosition,
};

pub struct TetrisBot {
    moves: Vec<Move>,
}

struct GridAnalysis {}

impl GridAnalysis {
    fn get_first_nonempty_row_index(grid: &Grid) -> usize {
        return grid
            .grid
            .iter()
            .enumerate()
            .filter(|(_i, row)| row.iter().any(|&x| x != EMPTY_CELL))
            .map(|(i_row, _row)| i_row)
            .next()
            .unwrap_or(GRID_HEIGHT as usize);
    }

    fn count_filled_rows(grid: &Grid) -> usize {
        return grid
            .grid
            .iter()
            .filter(|row| row.iter().all(|&x| x != EMPTY_CELL))
            .count();
    }

    fn count_gaps(grid: &Grid) -> usize {
        // a gap is when there are empty cells with filled cells above them
        // we calculate this metric per *column*.
        let mut total_gap_count = 0;
        for col_index in 0..GRID_WIDTH as usize {
            let mut n_zeros = 0;
            for row_index in (0..GRID_HEIGHT as usize).rev() {
                if n_zeros > 0 && grid.grid[row_index][col_index] != EMPTY_CELL {
                    total_gap_count += n_zeros;
                    n_zeros = 0;
                }
                if grid.grid[row_index][col_index] == EMPTY_CELL {
                    n_zeros += 1;
                }
            }
        }
        return total_gap_count;
    }
}

impl TetrisBot {
    pub fn new() -> Self {
        TetrisBot { moves: Vec::new() }
    }

    fn moves_to_str(moves: &Vec<Move>) -> String {
        return iter::once("<")
            .chain(
                moves
                    .iter()
                    .map(|mv| match mv {
                        Move::HardDrop => "",
                        Move::Simple(SimpleMove::Down) => "",
                        Move::Simple(SimpleMove::Left) => "L",
                        Move::Simple(SimpleMove::Right) => "R",
                        Move::Simple(SimpleMove::Rotate) => "S",
                    })
                    .rev(),
            )
            .chain(iter::once(">"))
            .collect::<Vec<&str>>()
            .join("");
    }

    fn as_moves_sequence(
        is_left: bool,
        n_horizontal_moves: i16,
        n_rotations: usize,
        n_drops: usize,
    ) -> Vec<Move> {
        // rotate before shift because sometimes the shift puts you into a position
        // where you can't rotate
        return (0..n_drops)
            .map(|_| Move::Simple(SimpleMove::Down))
            .chain((0..n_rotations).map(|_| Move::Simple(SimpleMove::Rotate)))
            .chain((0..n_horizontal_moves).map(|_| {
                if is_left {
                    Move::Simple(SimpleMove::Left)
                } else {
                    Move::Simple(SimpleMove::Right)
                }
            }))
            .chain(iter::once(Move::HardDrop))
            .rev() // because we use "pop" to fetch next move, which pops the last element of the vector
            .collect();
    }

    fn enumerate_options(
        grid: &Grid,
        original_shape: &ShapePosition,
    ) -> Vec<(ShapePosition, Vec<Move>)> {
        let mut result = Vec::new();
        let n_drops = 4;

        // these operations are not necessarily commutative?
        // eg sometimes you can't move then rotate, but you can rotate then move
        // but we can restrict ourselves to a subset of all possible moves for now

        // FIXME I think for some pieces you need to let them fall a bit to rotate them?
        for direction in [SimpleMove::Left, SimpleMove::Right] {
            for i_rotation in 0..(original_shape.n_rotations()) {
                for i_shift in 0..(GRID_WIDTH / 2 + 1) {
                    let direction_sign = if direction == SimpleMove::Left { -1 } else { 1 };
                    // always add a vertical shift to allow pieces to rotate
                    let shape =
                        original_shape.n_moves(i_shift * direction_sign, i_rotation, n_drops);
                    if grid.can_set_pixels(&shape.get_pixels(), &original_shape.get_pixels()) {
                        let pos_after_fall = Self::get_position_after_fall(&shape, grid);
                        let moves = Self::as_moves_sequence(
                            direction == SimpleMove::Left,
                            i_shift,
                            i_rotation,
                            n_drops as usize,
                        );
                        result.push((pos_after_fall, moves));
                    }
                }
            }
        }
        // debug print
        // println!("Options:");
        // result
        //     .iter()
        //     .map(|(_pos, mvs)| Self::moves_to_str(mvs))
        //     .for_each(|strg| println!("{}", strg));
        return result;
    }

    fn get_position_after_fall(original_shape: &ShapePosition, grid: &Grid) -> ShapePosition {
        let mut shape = original_shape.clone();
        loop {
            if !grid.can_set_pixels(
                &shape.moved_to(&SimpleMove::Down).get_pixels(),
                &original_shape.get_pixels(),
            ) {
                return shape;
            }
            shape = shape.moved_to(&SimpleMove::Down);
        }
    }

    fn grid_score(grid: &Grid) -> i32 {
        // highly reward full lines
        // penalize height increase and gap increase
        let index_of_first_nonempty_row = GridAnalysis::get_first_nonempty_row_index(grid) as i32;
        let n_filled_rows = GridAnalysis::count_filled_rows(grid) as i32;
        let n_gaps = GridAnalysis::count_gaps(grid) as i32;
        return n_filled_rows * 10 + index_of_first_nonempty_row - n_gaps;
    }

    fn decide_moves(original_grid: &Grid, current_shape: &ShapePosition) -> Vec<Move> {
        // enumerate all possible positions reachable from current state
        let mut grid = original_grid.clone();
        grid.unset_pixels(&current_shape.get_pixels());

        if let Some((_, best_moves)) = Self::enumerate_options(&grid, current_shape)
            .iter()
            .max_by_key(|(shape, moves)| {
                grid.set_pixels(&shape.get_pixels(), 1);
                let score = Self::grid_score(&grid);
                // for debugging
                println!("{} scores {}", Self::moves_to_str(moves), score);
                grid.unset_pixels(&shape.get_pixels());
                return score;
            })
        {
            return best_moves.to_vec();
        }
        return Vec::new();
    }

    pub fn update_policy(&mut self, grid: &Grid, current_shape: &ShapePosition) {
        self.moves = Self::decide_moves(grid, current_shape);
        println!("Chosen moves: {:?}", Self::moves_to_str(&self.moves));
    }

    pub fn pop_next_move(&mut self) -> Option<Move> {
        return self.moves.pop();
    }
}
