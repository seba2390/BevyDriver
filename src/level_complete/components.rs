use bevy::prelude::*;

/// Marker component for entities that belong to the level complete screen
#[derive(Component)]
pub struct OnLevelCompleteScreen;

/// All actions that can be triggered from level complete menu buttons
#[derive(Component)]
pub enum LevelCompleteButtonAction {
    RestartLevel,
    NextLevel,
    MainMenu,
    Quit,
}
