use bevy::prelude::*;

/// Marker component for entities that belong to the level menu screen
#[derive(Component)]
pub struct OnLevelMenuScreen;

/// All actions that can be triggered from level menu buttons
#[derive(Component)]
pub enum LevelMenuButtonAction {
    /// Play the selected level
    PlayLevel(usize),
    /// Return to start menu
    MainMenu,
}

/// Marker for the scrollable level list container
#[derive(Component)]
pub struct LevelListContainer;

/// Marker for individual level cards (stores level number)
#[derive(Component)]
pub struct LevelCard(pub usize);

/// Marker for the mini-map preview area within a level card (for future use)
#[derive(Component)]
pub struct LevelMiniMapPreview(pub usize);

/// Marker for the time display text within a level card
#[derive(Component)]
pub struct LevelTimeDisplay(pub usize);
