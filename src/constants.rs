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

// -- Resume Flag -- //
/// Resource to track whether we're resuming from pause (skip setup) or starting fresh
#[derive(Resource, Default)]
pub struct ResumeFromPause(pub bool);

// -- Game State -- //
/// Global game state enum for managing menu and gameplay transitions
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    /// Main menu screen (shown first on game start)
    #[default]
    StartMenu,
    /// Player name entry screen (shown when starting a new game)
    NewGameNameEntry,
    /// Load game menu (shown when loading a saved game)
    LoadGameMenu,
    /// Level selection menu (shows completed levels and times)
    LevelMenu,
    /// Active gameplay - race in progress
    Playing,
    /// Game is paused (overlay on top of gameplay)
    Paused,
    /// Level complete screen (shown when race finishes)
    LevelComplete,
}

// -- Bloom Settings -- //
/// Bloom intensity for the glow effect on unvisited road segments
/// Lower values for subtle localized glow
pub const BLOOM_INTENSITY: f32 = 0.45;

// -- Window Settings -- //
pub const WINDOW_WIDTH: u32 = 1300;
pub const WINDOW_HEIGHT: u32 = 800;
pub const LEFT_BOUNDARY: f32 = -(WINDOW_WIDTH as f32) / 2.0;
pub const RIGHT_BOUNDARY: f32 = (WINDOW_WIDTH as f32) / 2.0;
pub const TOP_BOUNDARY: f32 = (WINDOW_HEIGHT as f32) / 2.0;
pub const BOTTOM_BOUNDARY: f32 = -(WINDOW_HEIGHT as f32) / 2.0;

// -- Colors -- //
pub const GAME_BACKGROUND_COLOR: Color = Color::srgb(0.05, 0.05, 0.05);
