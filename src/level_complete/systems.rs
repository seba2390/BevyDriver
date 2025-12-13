use bevy::app::AppExit;
use bevy::prelude::*;

use crate::constants::{CurrentLevel, GameState};
use crate::level_complete::components::{LevelCompleteButtonAction, OnLevelCompleteScreen};
use crate::styles::colors::{BUTTON_HOVERED_COLOR, BUTTON_NORMAL_COLOR, BUTTON_PRESSED_COLOR, OVERLAY_BACKGROUND_COLOR};
use crate::styles::menu::{LARGE_BUTTON_WIDTH, button_node, button_text_style, column_centered, fullscreen_centered, title_style};

// ============================================================================
// Level Complete Menu Spawning
// ============================================================================

/// Spawns the level complete menu UI
pub fn spawn_level_complete_menu(mut commands: Commands) {
    commands
        .spawn(root_container())
        .with_children(|parent| {
            parent.spawn(menu_panel()).with_children(|parent| {
                spawn_title(parent);
                spawn_button(parent, "Restart Level", LevelCompleteButtonAction::RestartLevel);
                spawn_button(parent, "Next Level", LevelCompleteButtonAction::NextLevel);
                spawn_button(parent, "Main Menu", LevelCompleteButtonAction::MainMenu);
                spawn_button(parent, "Quit", LevelCompleteButtonAction::Quit);
            });
        });
}

fn root_container() -> impl Bundle {
    (fullscreen_centered(), BackgroundColor(OVERLAY_BACKGROUND_COLOR), OnLevelCompleteScreen)
}

fn menu_panel() -> impl Bundle {
    column_centered()
}

fn spawn_title(parent: &mut ChildSpawnerCommands) {
    parent.spawn((Text::new("Level Complete!"), title_style()));
}

fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str, action: LevelCompleteButtonAction) {
    parent
        .spawn((Button, button_node(LARGE_BUTTON_WIDTH), BackgroundColor(BUTTON_NORMAL_COLOR), action))
        .with_children(|parent| {
            parent.spawn((Text::new(label), button_text_style()));
        });
}

// ============================================================================
// Button Interaction
// ============================================================================

/// Handles button hover and press visual feedback
pub fn level_complete_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => BUTTON_PRESSED_COLOR.into(),
            Interaction::Hovered => BUTTON_HOVERED_COLOR.into(),
            Interaction::None => BUTTON_NORMAL_COLOR.into(),
        };
    }
}

/// Handles level complete menu button actions
pub fn level_complete_action(
    interaction_query: Query<
        (&Interaction, &LevelCompleteButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                LevelCompleteButtonAction::RestartLevel => {
                    // Restart the current level (level stays the same)
                    game_state.set(GameState::Playing);
                }
                LevelCompleteButtonAction::NextLevel => {
                    if current_level.0 >= 3 {
                        panic!("No more levels available! You've completed all 3 levels.");
                    }
                    current_level.0 += 1;
                    game_state.set(GameState::Playing);
                }
                LevelCompleteButtonAction::MainMenu => {
                    game_state.set(GameState::StartMenu);
                }
                LevelCompleteButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
            }
        }
    }
}

// ============================================================================
// Cleanup
// ============================================================================

/// Despawns all level complete menu entities and game entities when leaving this state
pub fn cleanup_level_complete_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<OnLevelCompleteScreen>>,
    game_query: Query<Entity, With<crate::start_menu::components::GameEntity>>,
) {
    for entity in &menu_query {
        commands.entity(entity).despawn();
    }
    // Also clean up the game entities that were visible behind the overlay
    for entity in &game_query {
        commands.entity(entity).despawn();
    }
}
