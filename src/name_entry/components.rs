use bevy::prelude::*;

/// Marker component for entities that belong to the name entry screen
#[derive(Component)]
pub struct OnNameEntryScreen;

/// Marker component for the text input field
#[derive(Component)]
pub struct NameInputText;

/// Resource to store the currently typed player name
#[derive(Resource, Default)]
pub struct PlayerNameInput(pub String);

/// All actions that can be triggered from name entry screen buttons
#[derive(Component)]
pub enum NameEntryButtonAction {
    StartGame,
    Back,
}
