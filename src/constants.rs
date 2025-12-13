use bevy::prelude::*;

// -- Current Level -- //
/// Resource to track the current level (1, 2, or 3)
#[derive(Resource)]
pub struct CurrentLevel(pub usize);

impl Default for CurrentLevel {
    fn default() -> Self {
        CurrentLevel(1)
    }
}

// -- Game State -- //
/// Global game state enum for managing menu and gameplay transitions
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    /// Main menu screen (shown first on game start)
    #[default]
    StartMenu,
    /// Active gameplay - race in progress
    Playing,
    /// Level complete screen (shown when race finishes)
    LevelComplete,
}

// -- Window Settings -- //
pub const WINDOW_WIDTH: u32 = 1000;
pub const WINDOW_HEIGHT: u32 = 700;
pub const LEFT_BOUNDARY: f32 = -(WINDOW_WIDTH as f32) / 2.0;
pub const RIGHT_BOUNDARY: f32 = (WINDOW_WIDTH as f32) / 2.0;
pub const TOP_BOUNDARY: f32 = (WINDOW_HEIGHT as f32) / 2.0;
pub const BOTTOM_BOUNDARY: f32 = -(WINDOW_HEIGHT as f32) / 2.0;
