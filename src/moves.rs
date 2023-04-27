use macroquad::prelude::{is_key_released, KeyCode};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimpleMove {
    Left,
    Right,
    Down,
    Rotate,
}

#[derive(Debug, Clone)]
pub enum Move {
    Simple(SimpleMove),
    // hard-drop means we drop the piece immediately as low as it can go
    // we set this one apart in the enum because a hard-drop move is the
    // only move whose computation depends on the state of the grid
    HardDrop,
}

impl Move {
    pub fn from_key_press() -> Option<Move> {
        if is_key_released(KeyCode::Left) {
            return Some(Move::Simple(SimpleMove::Left));
        } else if is_key_released(KeyCode::Right) {
            return Some(Move::Simple(SimpleMove::Right));
        } else if is_key_released(KeyCode::Up) {
            return Some(Move::Simple(SimpleMove::Rotate));
        } else if is_key_released(KeyCode::Down) {
            return Some(Move::Simple(SimpleMove::Down));
        } else if is_key_released(KeyCode::Space) {
            return Some(Move::HardDrop);
        }
        return None;
    }
}
