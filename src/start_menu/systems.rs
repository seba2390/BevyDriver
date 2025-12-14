use bevy::app::AppExit;
use bevy::prelude::*;

use crate::constants::GameState;
use crate::start_menu::components::{MenuButtonAction, OnMenuScreen};
use crate::styles::colors::{BUTTON_HOVERED_COLOR, BUTTON_NORMAL_COLOR, BUTTON_PRESSED_COLOR, MENU_BACKGROUND_COLOR};
use crate::styles::menu::{STANDARD_BUTTON_WIDTH, button_node, button_text_style, column_centered, fullscreen_centered, title_style};

// ============================================================================
// Menu Spawning
// ============================================================================

/// Spawns the main menu UI
pub fn spawn_menu(mut commands: Commands) {
    commands
        .spawn(root_container())
        .with_children(|parent| {
            parent.spawn(menu_panel()).with_children(|parent| {
                spawn_title(parent);
                spawn_button(parent, "New Game", MenuButtonAction::NewGame);
                spawn_button(parent, "Load Game", MenuButtonAction::LoadGame);
                spawn_button(parent, "Quit", MenuButtonAction::Quit);
            });
        });
}

fn root_container() -> impl Bundle {
    (fullscreen_centered(), BackgroundColor(MENU_BACKGROUND_COLOR), OnMenuScreen)
}

fn menu_panel() -> impl Bundle {
    column_centered()
}

fn spawn_title(parent: &mut ChildSpawnerCommands) {
    parent.spawn((Text::new("Bevy Driver"), title_style()));
}

fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str, action: MenuButtonAction) {
    parent
        .spawn((Button, button_node(STANDARD_BUTTON_WIDTH), BackgroundColor(BUTTON_NORMAL_COLOR), action))
        .with_children(|parent| {
            parent.spawn((Text::new(label), button_text_style()));
        });
}

// ============================================================================
// Button Interaction
// ============================================================================

/// Handles button hover and press visual feedback
pub fn button_system(
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
