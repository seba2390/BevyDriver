use bevy::app::AppExit;
use bevy::prelude::*;

use crate::constants::GameState;
use crate::start_menu::components::{MenuButtonAction, OnMenuScreen};
use crate::styles::colors::MENU_BACKGROUND_COLOR;
use crate::styles::menu::{
    column_centered, spawn_menu_container, spawn_standard_button, title_style,
};

// ============================================================================
// Menu Spawning
// ============================================================================

/// Spawns the main menu UI
pub fn spawn_menu(mut commands: Commands) {
    spawn_menu_container(&mut commands, OnMenuScreen, MENU_BACKGROUND_COLOR)
        .with_children(|parent| {
            parent.spawn(column_centered()).with_children(|parent| {
                parent.spawn((Text::new("Bevy Driver"), title_style()));
                spawn_standard_button(parent, "New Game", MenuButtonAction::NewGame);
                spawn_standard_button(parent, "Load Game", MenuButtonAction::LoadGame);
                spawn_standard_button(parent, "Quit", MenuButtonAction::Quit);
            });
        });
}

// ============================================================================
// Button Actions
// ============================================================================

/// Handles menu button actions (NewGame, LoadGame, Quit)
pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::NewGame => game_state.set(GameState::NewGameNameEntry),
                MenuButtonAction::LoadGame => game_state.set(GameState::LoadGameMenu),
                MenuButtonAction::Quit => { app_exit_writer.write(AppExit::Success); }
            }
        }
    }
}
