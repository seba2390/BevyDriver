use bevy::app::AppExit;
use bevy::prelude::*;

use crate::constants::{CurrentLevel, GameState};
use crate::hud::components::RaceState;
use crate::level_complete::components::{LevelCompleteButtonAction, OnLevelCompleteScreen};
use crate::save::{save_to_file, CurrentSave};
use crate::styles::colors::{BUTTON_HOVERED_COLOR, BUTTON_NORMAL_COLOR, BUTTON_PRESSED_COLOR, OVERLAY_BACKGROUND_COLOR};
use crate::styles::menu::{LARGE_BUTTON_WIDTH, button_node, button_text_style, column_centered, fullscreen_centered, title_style};

// ============================================================================
// Level Complete Menu Spawning
// ============================================================================

/// Spawns the level complete menu UI and auto-saves progress
pub fn spawn_level_complete_menu(
    mut commands: Commands,
    race_state: Res<RaceState>,
    current_level: Res<CurrentLevel>,
    mut current_save: ResMut<CurrentSave>,
) {
    // Auto-save progress if we have an active save
    let mut new_best = false;
    if let Some(save_data) = current_save.get_mut() {
        if let Some(final_time) = race_state.final_time {
            new_best = save_data.record_level_completion(current_level.0, final_time);
            // Save to file
            let _ = save_to_file(save_data);
        }
    }

    let final_time_str = race_state
        .final_time
        .map(|t| format!("{:.2}s", t))
        .unwrap_or_else(|| "N/A".to_string());

    commands
        .spawn(root_container())
        .with_children(|parent| {
            parent.spawn(menu_panel()).with_children(|parent| {
                spawn_title(parent);
                spawn_time_display(parent, &final_time_str, new_best);
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

fn spawn_time_display(parent: &mut ChildSpawnerCommands, time_str: &str, is_new_best: bool) {
    parent.spawn((
        Text::new(format!("Time: {}", time_str)),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

    if is_new_best {
        parent.spawn((
            Text::new("New Best Time!"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.3, 1.0, 0.3)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
    } else {
        parent.spawn((
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                height: Val::Px(24.0),
                ..default()
            },
        ));
    }
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
                    // Levels 1-3 are hardcoded, levels 4+ are randomly generated
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
