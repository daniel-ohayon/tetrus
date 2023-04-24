pub static MUSIC_BYTES: &'static [u8] = include_bytes!("music.ogg");

// following https://tetris.wiki/Playfield  -- 20/10
pub const GRID_HEIGHT: i16 = 20;
pub const GRID_WIDTH: i16 = 10;

// pixel drawing constants
pub const BLOCK_SIZE: i16 = 30;
pub const CELL_BORDER: f32 = 2.;

// gameplay/debounce time constants
pub const USER_MOVE_DEBOUNCE_MS: u128 = 100;
