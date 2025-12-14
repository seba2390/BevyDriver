use bevy::app::AppExit;
use bevy::prelude::*;

use crate::constants::{GameState, ResumeFromPause};
use crate::pause_menu::components::{OnPauseMenuScreen, PauseMenuButtonAction};
use crate::start_menu::components::GameEntity;
use crate::styles::colors::OVERLAY_BACKGROUND_COLOR;
use crate::styles::menu::{
    column_centered, spawn_menu_container, spawn_button_with_width, title_style, LARGE_BUTTON_WIDTH,
};

// ============================================================================
// Pause Menu Spawning
// ============================================================================

/// Spawns the pause menu UI overlay
pub fn spawn_pause_menu(mut commands: Commands) {
    spawn_menu_container(&mut commands, OnPauseMenuScreen, OVERLAY_BACKGROUND_COLOR)
        .with_children(|parent| {
            parent.spawn(column_centered()).with_children(|parent| {
                parent.spawn((Text::new("Paused"), title_style()));
                spawn_button_with_width(parent, "Resume", PauseMenuButtonAction::Resume, LARGE_BUTTON_WIDTH);
                spawn_button_with_width(parent, "Main Menu", PauseMenuButtonAction::MainMenu, LARGE_BUTTON_WIDTH);
                spawn_button_with_width(parent, "Quit", PauseMenuButtonAction::Quit, LARGE_BUTTON_WIDTH);
            });
        });
}

// ============================================================================
// Pause Input Handling
// ============================================================================

/// Handles the Escape key to toggle pause state during gameplay
pub fn handle_pause_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
    }
}

/// Handles the Escape key to resume from pause menu
pub fn handle_resume_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut resume_flag: ResMut<ResumeFromPause>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        resume_flag.0 = true;
        game_state.set(GameState::Playing);
    }
}

// ============================================================================
// Button Actions
// ============================================================================

/// Handles pause menu button actions
pub fn pause_menu_action(
    interaction_query: Query<
        (&Interaction, &PauseMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    game_entities: Query<Entity, With<GameEntity>>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut resume_flag: ResMut<ResumeFromPause>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                PauseMenuButtonAction::Resume => {
                    resume_flag.0 = true;
                    game_state.set(GameState::Playing);
                }
                PauseMenuButtonAction::MainMenu => {
                    // Clean up game entities when returning to main menu
                    for entity in &game_entities {
                        commands.entity(entity).despawn();
                    }
                    game_state.set(GameState::StartMenu);
                }
                PauseMenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
            }
        }
    }
}
