use bevy::app::AppExit;
use bevy::prelude::*;

use crate::constants::{CurrentLevel, GameState};
use crate::hud::components::RaceState;
use crate::level_complete::components::{LevelCompleteButtonAction, OnLevelCompleteScreen};
use crate::level_complete::constants::{
    NEW_BEST_FONT_SIZE, NEW_BEST_MARGIN, PLACEHOLDER_HEIGHT, TIME_DISPLAY_FONT_SIZE, TIME_DISPLAY_MARGIN,
};
use crate::save::{save_to_file, CurrentSave};
use crate::styles::colors::{
    MENU_TEXT_COLOR, OVERLAY_BACKGROUND_COLOR, SUCCESS_TEXT_COLOR,
};
use crate::styles::menu::{
    column_centered, spawn_menu_container, spawn_button_with_width, title_style, LARGE_BUTTON_WIDTH,
};

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

    spawn_menu_container(&mut commands, OnLevelCompleteScreen, OVERLAY_BACKGROUND_COLOR)
        .with_children(|parent| {
            parent.spawn(column_centered()).with_children(|parent| {
                parent.spawn((Text::new("Level Complete!"), title_style()));
                spawn_time_display(parent, &final_time_str, new_best);
                spawn_button_with_width(parent, "Restart Level", LevelCompleteButtonAction::RestartLevel, LARGE_BUTTON_WIDTH);
                spawn_button_with_width(parent, "Next Level", LevelCompleteButtonAction::NextLevel, LARGE_BUTTON_WIDTH);
                spawn_button_with_width(parent, "Main Menu", LevelCompleteButtonAction::MainMenu, LARGE_BUTTON_WIDTH);
                spawn_button_with_width(parent, "Quit", LevelCompleteButtonAction::Quit, LARGE_BUTTON_WIDTH);
            });
        });
}

fn spawn_time_display(parent: &mut ChildSpawnerCommands, time_str: &str, is_new_best: bool) {
    parent.spawn((
        Text::new(format!("Time: {}", time_str)),
        TextFont {
            font_size: TIME_DISPLAY_FONT_SIZE,
            ..default()
        },
        TextColor(MENU_TEXT_COLOR),
        Node {
            margin: UiRect::bottom(Val::Px(TIME_DISPLAY_MARGIN)),
            ..default()
        },
    ));

    if is_new_best {
        parent.spawn((
            Text::new("New Best Time!"),
            TextFont {
                font_size: NEW_BEST_FONT_SIZE,
                ..default()
            },
            TextColor(SUCCESS_TEXT_COLOR),
            Node {
                margin: UiRect::bottom(Val::Px(NEW_BEST_MARGIN)),
                ..default()
            },
        ));
    } else {
        parent.spawn((
            Node {
                margin: UiRect::bottom(Val::Px(NEW_BEST_MARGIN)),
                height: Val::Px(PLACEHOLDER_HEIGHT),
                ..default()
            },
        ));
    }
}

// ============================================================================
// Button Actions
// ============================================================================

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
