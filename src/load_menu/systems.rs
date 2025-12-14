use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::constants::{CurrentLevel, GameState};
use crate::load_menu::components::{DeleteConfirmation, DeleteConfirmButtonAction, DeleteConfirmationOverlay, LoadMenuButtonAction, MenuPanel, NoSavesMessage, OnLoadMenuScreen, SaveSlot, SaveSlotRow, SavesListContainer};
use crate::load_menu::constants::*;
use crate::save::{delete_save_file, list_saves, load_from_file, CurrentSave, SaveData};
use crate::styles::colors::{
    BUTTON_NORMAL_COLOR, MENU_BACKGROUND_COLOR, MENU_TEXT_COLOR, OVERLAY_BACKGROUND_COLOR,
    SECONDARY_TEXT_COLOR,
};
use crate::styles::menu::{
    column_centered, spawn_menu_container, spawn_standard_button, title_style,
    button_text_style, no_saves_message_bundle, ButtonColors, BUTTON_HEIGHT,
};

// ============================================================================
// Load Menu Screen Spawning
// ============================================================================

/// Spawns the load menu screen UI
pub fn spawn_load_menu(mut commands: Commands) {
    // Initialize delete confirmation resource
    commands.insert_resource(DeleteConfirmation::default());

    let saves = list_saves().unwrap_or_default();

    spawn_menu_container(&mut commands, OnLoadMenuScreen, MENU_BACKGROUND_COLOR)
        .with_children(|parent| {
            parent.spawn((column_centered(), MenuPanel)).with_children(|parent| {
                parent.spawn((Text::new("Load Game"), title_style()));

                if saves.is_empty() {
                    parent.spawn((no_saves_message_bundle(), NoSavesMessage));
                } else {
                    spawn_saves_list(parent, &saves);
                }

                spawn_standard_button(parent, "Back", LoadMenuButtonAction::Back);
            });
        });
}

fn spawn_saves_list(parent: &mut ChildSpawnerCommands, saves: &[SaveData]) {
    // Scrollable container for save slots
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                max_height: Val::Px(SCROLL_CONTAINER_HEIGHT),
                overflow: Overflow::scroll_y(),
                margin: UiRect::vertical(Val::Px(SAVES_LIST_MARGIN)),
                ..default()
            },
            SavesListContainer,
        ))
        .with_children(|scroll_parent| {
            for save in saves {
                spawn_save_slot(scroll_parent, save);
            }
        });
}

fn spawn_save_slot(parent: &mut ChildSpawnerCommands, save: &SaveData) {
    let filename = save.filename();
    let last_played = save.last_played.format("%Y-%m-%d %H:%M").to_string();
    let levels_completed = save.level_times.len();
    let highest_level = save.highest_level_unlocked;

    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(SAVE_SLOT_SPACING)),
                ..default()
            },
            SaveSlotRow(filename.clone()),
        ))
        .with_children(|row| {
            // Main save slot button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(SAVE_SLOT_WIDTH),
                    height: Val::Px(SAVE_SLOT_HEIGHT),
                    padding: UiRect::all(Val::Px(SAVE_SLOT_PADDING)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                BackgroundColor(BUTTON_NORMAL_COLOR),
                SaveSlot(filename.clone()),
            ))
            .with_children(|slot| {
                // Player name
                slot.spawn((
                    Text::new(&save.player_name),
                    TextFont {
                        font_size: PLAYER_NAME_FONT_SIZE,
                        ..default()
                    },
                    TextColor(MENU_TEXT_COLOR),
                ));

                // Stats line
                slot.spawn((
                    Text::new(format!(
                        "Level {} | {} completed | Last: {}",
                        highest_level, levels_completed, last_played
                    )),
                    TextFont {
                        font_size: STATS_LINE_FONT_SIZE,
                        ..default()
                    },
                    TextColor(SECONDARY_TEXT_COLOR),
                ));
            });

            // Delete button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(DELETE_BUTTON_SIZE),
                    height: Val::Px(DELETE_BUTTON_SIZE),
                    margin: UiRect::left(Val::Px(SAVE_SLOT_SPACING)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(DELETE_BUTTON_COLOR),
                ButtonColors::new(DELETE_BUTTON_COLOR, DELETE_BUTTON_HOVER, DELETE_BUTTON_PRESSED),
                DeleteButton(filename),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("X"),
                    TextFont {
                        font_size: DELETE_BUTTON_FONT_SIZE,
                        ..default()
                    },
                    TextColor(MENU_TEXT_COLOR),
                ));
            });
        });
}

/// Marker for delete buttons, stores the save filename
#[derive(Component)]
pub struct DeleteButton(pub String);

// ============================================================================
// Button Actions
// ============================================================================

/// Handles clicking on a save slot to load the game
pub fn handle_save_slot_click(
    interaction_query: Query<
        (&Interaction, &SaveSlot),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_save: ResMut<CurrentSave>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for (interaction, save_slot) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Load the save file
            if let Ok(save_data) = load_from_file(&save_slot.0) {
                current_level.0 = save_data.highest_level_unlocked;
                current_save.set(save_data);
                // Navigate to level menu instead of directly to gameplay
                game_state.set(GameState::LevelMenu);
            }
        }
    }
}

/// Handles clicking the delete button - shows confirmation overlay
pub fn handle_delete_click(
    interaction_query: Query<
        (&Interaction, &DeleteButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    mut delete_confirmation: ResMut<DeleteConfirmation>,
    _saves: Query<&SaveSlot>,
    existing_overlay: Query<Entity, With<DeleteConfirmationOverlay>>,
) {
    for (interaction, delete_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Don't open another overlay if one exists
            if !existing_overlay.is_empty() {
                continue;
            }

            // Find the player name for this save
            let player_name = list_saves()
                .unwrap_or_default()
                .into_iter()
                .find(|s| s.filename() == delete_button.0)
                .map(|s| s.player_name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            // Store which save we're confirming deletion for
            delete_confirmation.filename = Some(delete_button.0.clone());
            delete_confirmation.player_name = Some(player_name.clone());

            // Spawn confirmation overlay
            spawn_delete_confirmation_overlay(&mut commands, &player_name);
        }
    }
}

/// Spawns the delete confirmation overlay
fn spawn_delete_confirmation_overlay(commands: &mut Commands, player_name: &str) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(OVERLAY_BACKGROUND_COLOR),
            DeleteConfirmationOverlay,
            GlobalZIndex(100),
            FocusPolicy::Block,
        ))
        .with_children(|parent| {
            // Confirmation message
            parent.spawn((
                Text::new(format!("Are you sure you want to delete \"{}\"?", player_name)),
                TextFont {
                    font_size: CONFIRM_MESSAGE_FONT_SIZE,
                    ..default()
                },
                TextColor(MENU_TEXT_COLOR),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    margin: UiRect::bottom(Val::Px(NO_SAVES_MESSAGE_MARGIN)),
                    ..default()
                },
            ));

            // Button row
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|buttons| {
                    // Delete button (red)
                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(CONFIRM_BUTTON_WIDTH),
                                height: Val::Px(BUTTON_HEIGHT),
                                margin: UiRect::right(Val::Px(CONFIRM_BUTTON_SPACING)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(DELETE_BUTTON_COLOR),
                            ButtonColors::new(DELETE_BUTTON_COLOR, DELETE_BUTTON_HOVER, DELETE_BUTTON_PRESSED),
                            DeleteConfirmButtonAction::ConfirmDelete,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("Delete"),
                                button_text_style(),
                            ));
                        });

                    // Cancel button
                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(CONFIRM_BUTTON_WIDTH),
                                height: Val::Px(BUTTON_HEIGHT),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(BUTTON_NORMAL_COLOR),
                            DeleteConfirmButtonAction::CancelDelete,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("Cancel"),
                                button_text_style(),
                            ));
                        });
                });
        });
}

/// Handles confirmation dialog button actions
pub fn handle_delete_confirm_action(
    interaction_query: Query<
        (&Interaction, &DeleteConfirmButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    overlay_query: Query<Entity, With<DeleteConfirmationOverlay>>,
    save_slot_rows: Query<(Entity, &SaveSlotRow)>,
    saves_list_container: Query<Entity, With<SavesListContainer>>,
    menu_panel: Query<Entity, With<MenuPanel>>,
    mut delete_confirmation: ResMut<DeleteConfirmation>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                DeleteConfirmButtonAction::ConfirmDelete => {
                    // Delete the save file and despawn the row
                    if let Some(filename) = &delete_confirmation.filename {
                        let _ = delete_save_file(filename);

                        // Find and despawn the save slot row
                        for (entity, row) in &save_slot_rows {
                            if &row.0 == filename {
                                commands.entity(entity).despawn();
                                break;
                            }
                        }

                        // Check if this was the last save (only 1 row existed before deletion)
                        if save_slot_rows.iter().count() == 1 {
                            // Despawn the saves list container
                            for entity in &saves_list_container {
                                commands.entity(entity).despawn();
                            }

                            // Spawn the "no saves" message as a child of the menu panel
                            // Insert at index 1 (after title, before Back button) to match spawn_load_menu order
                            if let Ok(panel_entity) = menu_panel.single() {
                                let message_entity = commands.spawn((
                                    no_saves_message_bundle(),
                                    NoSavesMessage,
                                )).id();

                                // Insert at index 1 (after title at index 0, before Back button)
                                // This matches the order in spawn_load_menu: title, no_saves_message, back_button
                                commands.entity(panel_entity).insert_children(1, &[message_entity]);
                            }
                        }
                    }

                    // Clear confirmation state
                    delete_confirmation.filename = None;
                    delete_confirmation.player_name = None;

                    // Despawn overlay
                    for entity in &overlay_query {
                        commands.entity(entity).despawn();
                    }
                }
                DeleteConfirmButtonAction::CancelDelete => {
                    // Clear confirmation state
                    delete_confirmation.filename = None;
                    delete_confirmation.player_name = None;

                    // Just despawn the overlay
                    for entity in &overlay_query {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

/// Handles load menu button actions (Back)
pub fn load_menu_action(
    interaction_query: Query<
        (&Interaction, &LoadMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                LoadMenuButtonAction::Back => {
                    game_state.set(GameState::StartMenu);
                }
            }
        }
    }
}

/// Cleanup when leaving the load menu screen
pub fn cleanup_load_menu(mut commands: Commands) {
    commands.remove_resource::<DeleteConfirmation>();
}
