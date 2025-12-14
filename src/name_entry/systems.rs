use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;

use crate::constants::{CurrentLevel, GameState};
use crate::name_entry::components::{NameEntryButtonAction, NameInputText, OnNameEntryScreen, PlayerNameInput};
use crate::name_entry::constants::*;
use crate::save::{save_exists, save_to_file, CurrentSave, SaveData};
use crate::styles::colors::{
    BUTTON_HOVERED_COLOR, BUTTON_NORMAL_COLOR, BUTTON_PRESSED_COLOR, MENU_BACKGROUND_COLOR, MENU_TEXT_COLOR,
};
use crate::styles::menu::{
    STANDARD_BUTTON_WIDTH, button_node, button_text_style, column_centered, fullscreen_centered, title_style,
    BUTTON_FONT_SIZE,
};

// ============================================================================
// Name Entry Screen Spawning
// ============================================================================

/// Spawns the name entry screen UI
pub fn spawn_name_entry(mut commands: Commands) {
    // Initialize the player name input resource
    commands.insert_resource(PlayerNameInput::default());

    commands
        .spawn(root_container())
        .with_children(|parent| {
            parent.spawn(menu_panel()).with_children(|parent| {
                spawn_title(parent);
                spawn_subtitle(parent);
                spawn_input_field(parent);
                spawn_error_text(parent);
                spawn_button(parent, "Start Game", NameEntryButtonAction::StartGame);
                spawn_button(parent, "Back", NameEntryButtonAction::Back);
            });
        });
}

fn root_container() -> impl Bundle {
    (fullscreen_centered(), BackgroundColor(MENU_BACKGROUND_COLOR), OnNameEntryScreen)
}

fn menu_panel() -> impl Bundle {
    column_centered()
}

fn spawn_title(parent: &mut ChildSpawnerCommands) {
    parent.spawn((Text::new("New Game"), title_style()));
}

fn spawn_subtitle(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new("Enter your name:"),
        TextFont {
            font_size: SUBTITLE_FONT_SIZE,
            ..default()
        },
        TextColor(MENU_TEXT_COLOR),
        Node {
            margin: UiRect::bottom(Val::Px(SUBTITLE_MARGIN)),
            ..default()
        },
    ));
}

fn spawn_input_field(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Px(INPUT_FIELD_WIDTH),
                height: Val::Px(INPUT_FIELD_HEIGHT),
                margin: UiRect::bottom(Val::Px(INPUT_FIELD_MARGIN)),
                padding: UiRect::horizontal(Val::Px(INPUT_FIELD_PADDING)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(INPUT_FIELD_BORDER_WIDTH)),
                ..default()
            },
            BackgroundColor(INPUT_BACKGROUND_COLOR),
            BorderColor::all(INPUT_BORDER_COLOR),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Type your name..."),
                TextFont {
                    font_size: BUTTON_FONT_SIZE,
                    ..default()
                },
                TextColor(PLACEHOLDER_COLOR),
                NameInputText,
            ));
        });
}

fn spawn_error_text(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new(""),
        TextFont {
            font_size: ERROR_TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(ERROR_TEXT_COLOR),
        Node {
            margin: UiRect::bottom(Val::Px(ERROR_TEXT_MARGIN)),
            height: Val::Px(ERROR_TEXT_HEIGHT),
            ..default()
        },
        ErrorText,
    ));
}

/// Marker for the error text display
#[derive(Component)]
pub struct ErrorText;

fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str, action: NameEntryButtonAction) {
    parent
        .spawn((Button, button_node(STANDARD_BUTTON_WIDTH), BackgroundColor(BUTTON_NORMAL_COLOR), action))
        .with_children(|parent| {
            parent.spawn((Text::new(label), button_text_style()));
        });
}

// ============================================================================
// Keyboard Input Handling
// ============================================================================

/// Handles keyboard input for the player name
pub fn handle_name_input(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut player_name: ResMut<PlayerNameInput>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<NameInputText>>,
) {
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match &event.logical_key {
            Key::Backspace => {
                player_name.0.pop();
            }
            Key::Character(c) => {
                // Only allow alphanumeric characters, spaces, underscores, and hyphens
                let valid_chars: String = c.chars()
                    .filter(|ch| ch.is_alphanumeric() || *ch == ' ' || *ch == '_' || *ch == '-')
                    .collect();

                if player_name.0.len() + valid_chars.len() <= MAX_NAME_LENGTH {
                    player_name.0.push_str(&valid_chars);
                }
            }
            Key::Space => {
                if player_name.0.len() < MAX_NAME_LENGTH && !player_name.0.is_empty() {
                    player_name.0.push(' ');
                }
            }
            _ => {}
        }
    }

    // Update the displayed text
    if let Ok((mut text, mut color)) = text_query.single_mut() {
        if player_name.0.is_empty() {
            *text = Text::new("Type your name...");
            *color = TextColor(PLACEHOLDER_COLOR);
        } else {
            *text = Text::new(format!("{}_", &player_name.0));
            *color = TextColor(MENU_TEXT_COLOR);
        }
    }
}

// ============================================================================
// Button Interaction
// ============================================================================

/// Handles button hover and press visual feedback
pub fn name_entry_button_system(
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

/// Handles name entry screen button actions
pub fn name_entry_action(
    interaction_query: Query<
        (&Interaction, &NameEntryButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_save: ResMut<CurrentSave>,
    mut current_level: ResMut<CurrentLevel>,
    player_name: Res<PlayerNameInput>,
    mut error_text_query: Query<&mut Text, With<ErrorText>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                NameEntryButtonAction::StartGame => {
                    let name = player_name.0.trim().to_string();

                    // Validate name
                    if name.is_empty() {
                        if let Ok(mut text) = error_text_query.single_mut() {
                            *text = Text::new("Please enter a name");
                        }
                        continue;
                    }

                    // Check if save already exists
                    if save_exists(&name) {
                        if let Ok(mut text) = error_text_query.single_mut() {
                            *text = Text::new("Name already exists! Choose another.");
                        }
                        continue;
                    }

                    // Create new save
                    let save_data = SaveData::new(name);

                    // Save to file
                    if let Err(e) = save_to_file(&save_data) {
                        if let Ok(mut text) = error_text_query.single_mut() {
                            *text = Text::new(format!("Failed to save: {}", e));
                        }
                        continue;
                    }

                    // Set current save and level
                    current_level.0 = 1;
                    current_save.set(save_data);
                    game_state.set(GameState::Playing);
                }
                NameEntryButtonAction::Back => {
                    game_state.set(GameState::StartMenu);
                }
            }
        }
    }
}

/// Cleanup when leaving the name entry screen
pub fn cleanup_name_entry(mut commands: Commands) {
    commands.remove_resource::<PlayerNameInput>();
}
