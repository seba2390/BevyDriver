use bevy::prelude::*;

/// Marker component for entities that belong to the menu screen
#[derive(Component)]
pub struct OnMenuScreen;

/// Marker component for gameplay entities that should be despawned when leaving the game
#[derive(Component)]
pub struct GameEntity;

/// All actions that can be triggered from menu button clicks
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}
