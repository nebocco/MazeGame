//! Define constants which are widely used in the crate.

/// arina size definition
pub const CELL_SIZE: f32 = 16.;
pub const DEFAULT_GRID_SIZE: f32 = 16.;
pub const DEFAULT_MAP_SIZE: f32 = DEFAULT_GRID_SIZE * CELL_SIZE;

/// frames within which the player can't move after a move
pub const FREEZE_TIME: u32 = 10;