use bevy::prelude::*;

/// Marker component for entities that belong to the pause menu screen
#[derive(Component)]
pub struct OnPauseMenuScreen;

/// All actions that can be triggered from pause menu buttons
#[derive(Component)]
pub enum PauseMenuButtonAction {
    Resume,
    LevelMenu,
    MainMenu,
    Quit,
}
