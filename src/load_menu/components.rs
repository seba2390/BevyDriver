use bevy::prelude::*;

/// Marker component for entities that belong to the load menu screen
#[derive(Component)]
pub struct OnLoadMenuScreen;

/// Marker component for the menu panel (contains title, saves list, and back button)
#[derive(Component)]
pub struct MenuPanel;

/// Marker component for a save slot button, stores the save filename
#[derive(Component)]
pub struct SaveSlot(pub String);

/// Marker component for a save slot row (contains both the slot button and delete button)
#[derive(Component)]
pub struct SaveSlotRow(pub String);

/// Marker component for the saves list container
#[derive(Component)]
pub struct SavesListContainer;

/// Marker component for the "no saves" message
#[derive(Component)]
pub struct NoSavesMessage;

/// Marker component for the delete confirmation overlay
#[derive(Component)]
pub struct DeleteConfirmationOverlay;

/// All actions that can be triggered from load menu buttons
#[derive(Component)]
pub enum LoadMenuButtonAction {
    Back,
}

/// Actions for the delete confirmation dialog
#[derive(Component)]
pub enum DeleteConfirmButtonAction {
    ConfirmDelete,
    CancelDelete,
}

/// Resource to track which save is selected for deletion confirmation
#[derive(Resource, Default)]
pub struct DeleteConfirmation {
    pub filename: Option<String>,
    pub player_name: Option<String>,
}
