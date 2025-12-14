use bevy::prelude::*;
use bevy_scrollbar::{Scrollbar, ScrollSpeed};

use crate::constants::{CurrentLevel, GameState, ResumeFromPause};
use crate::level_menu::components::{
    LevelCard, LevelListContainer, LevelMenuButtonAction, LevelMiniMapPreview,
    LevelTimeDisplay, OnLevelMenuScreen,
};
use crate::level_menu::constants::*;
use crate::level_menu::minimap::MinimapCache;
use crate::save::CurrentSave;
use crate::styles::colors::{
    BUTTON_NORMAL_COLOR, MENU_BACKGROUND_COLOR, MENU_TEXT_COLOR, SECONDARY_TEXT_COLOR,
    SUCCESS_TEXT_COLOR,
};
use crate::styles::menu::{
    column_centered, spawn_button_with_width, spawn_menu_container, title_style,
    LARGE_BUTTON_WIDTH,
};

// ============================================================================
// Level Menu Spawning
// ============================================================================

/// Spawns the level menu screen UI
pub fn spawn_level_menu(mut commands: Commands, current_save: Res<CurrentSave>) {
    let save_data = current_save.0.as_ref();
    let player_name = save_data.map(|s| s.player_name.as_str()).unwrap_or("Player");
    let highest_level = save_data.map(|s| s.highest_level_unlocked).unwrap_or(1);

    spawn_menu_container(&mut commands, OnLevelMenuScreen, MENU_BACKGROUND_COLOR)
        .with_children(|parent| {
            parent.spawn(column_centered()).with_children(|parent| {
                // Title with player name
                parent.spawn((
                    Text::new(format!("{}'s Levels", player_name)),
                    title_style(),
                ));

                // Scrollable level list
                spawn_level_list(parent, save_data, highest_level);

                // Main Menu button
                spawn_button_with_width(
                    parent,
                    "Main Menu",
                    LevelMenuButtonAction::MainMenu,
                    LARGE_BUTTON_WIDTH,
                );
            });
        });
}

/// Spawns the scrollable list of level cards with a scrollbar
fn spawn_level_list(
    parent: &mut ChildSpawnerCommands,
    save_data: Option<&crate::save::SaveData>,
    highest_level: usize,
) {
    // Calculate if scrolling is needed based on content height vs container height
    // Each card is LEVEL_CARD_HEIGHT + LEVEL_CARD_SPACING (except the last one)
    let content_height = highest_level as f32 * LEVEL_CARD_HEIGHT
        + (highest_level.saturating_sub(1)) as f32 * LEVEL_CARD_SPACING;
    let needs_scrollbar = content_height > LEVEL_LIST_HEIGHT;

    // Container for the scrollable content and scrollbar (siblings)
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Stretch,
            margin: UiRect::vertical(Val::Px(LEVEL_LIST_MARGIN)),
            height: Val::Px(LEVEL_LIST_HEIGHT),
            ..default()
        })
        .with_children(|container| {
            // Scrollable content
            let scrollable_id = container
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        height: Val::Percent(100.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    LevelListContainer,
                    ScrollSpeed(SCROLL_SPEED),
                ))
                .with_children(|scroll_parent| {
                    // Show all unlocked levels (1 through highest_level)
                    for level in 1..=highest_level {
                        let best_time = save_data.and_then(|s| s.level_times.get(&level).copied());
                        spawn_level_card(scroll_parent, level, best_time);
                    }
                })
                .id();

            // Only spawn scrollbar if content exceeds container height
            // bevy_scrollbar crashes when max scroll < 0 (content smaller than container)
            if needs_scrollbar {
                container.spawn((
                    Scrollbar { scrollable: scrollable_id },
                    Node {
                        width: Val::Px(SCROLLBAR_WIDTH),
                        height: Val::Percent(100.0),
                        margin: UiRect::left(Val::Px(SCROLLBAR_MARGIN)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                    BorderRadius::all(Val::Px(SCROLLBAR_WIDTH / 2.0)),
                ));
            }
        });
}

/// Spawns a single level card with number, status, time, and mini-map placeholder
fn spawn_level_card(parent: &mut ChildSpawnerCommands, level: usize, best_time: Option<f32>) {
    let is_completed = best_time.is_some();

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(LEVEL_CARD_WIDTH),
                height: Val::Px(LEVEL_CARD_HEIGHT),
                margin: UiRect::bottom(Val::Px(LEVEL_CARD_SPACING)),
                padding: UiRect::all(Val::Px(LEVEL_CARD_PADDING)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BUTTON_NORMAL_COLOR),
            LevelCard(level),
            LevelMenuButtonAction::PlayLevel(level),
        ))
        .with_children(|card| {
            // Level number column
            card.spawn((
                Node {
                    width: Val::Px(LEVEL_NUMBER_WIDTH),
                    ..default()
                },
            ))
            .with_children(|col| {
                col.spawn((
                    Text::new(format!("Level {}", level)),
                    TextFont {
                        font_size: LEVEL_NUMBER_FONT_SIZE,
                        ..default()
                    },
                    TextColor(MENU_TEXT_COLOR),
                ));
            });

            // Status and time column (flexible width)
            card.spawn((
                Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|col| {
                // Status text
                let (status_text, status_color) = if is_completed {
                    ("Completed", SUCCESS_TEXT_COLOR)
                } else {
                    ("Not completed", SECONDARY_TEXT_COLOR)
                };

                col.spawn((
                    Text::new(status_text),
                    TextFont {
                        font_size: LEVEL_STATUS_FONT_SIZE,
                        ..default()
                    },
                    TextColor(status_color),
                ));

                // Time display
                let time_text = if let Some(time) = best_time {
                    format_time(time)
                } else {
                    "-".to_string()
                };

                col.spawn((
                    Text::new(time_text),
                    TextFont {
                        font_size: LEVEL_TIME_FONT_SIZE,
                        ..default()
                    },
                    TextColor(if is_completed {
                        MENU_TEXT_COLOR
                    } else {
                        SECONDARY_TEXT_COLOR
                    }),
                    LevelTimeDisplay(level),
                ));
            });

            // Mini-map preview container (populated by minimap rendering system)
            card.spawn((
                Node {
                    width: Val::Px(MINI_MAP_WIDTH),
                    height: Val::Px(MINI_MAP_HEIGHT),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(SECONDARY_TEXT_COLOR),
                BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
                LevelMiniMapPreview(level),
            ));
        });
}

/// Formats time in seconds to MM:SS.ss format
fn format_time(seconds: f32) -> String {
    let mins = (seconds / 60.0).floor() as u32;
    let secs = seconds % 60.0;
    format!("{:02}:{:05.2}", mins, secs)
}

// ============================================================================
// Button Actions
// ============================================================================

/// Handles level menu button actions
pub fn level_menu_action(
    interaction_query: Query<
        (&Interaction, &LevelMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
    mut resume_flag: ResMut<ResumeFromPause>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                LevelMenuButtonAction::PlayLevel(level) => {
                    current_level.0 = *level;
                    // Ensure we start fresh (not resuming)
                    resume_flag.0 = false;
                    game_state.set(GameState::Playing);
                }
                LevelMenuButtonAction::MainMenu => {
                    game_state.set(GameState::StartMenu);
                }
            }
        }
    }
}

// ============================================================================
// Minimap Preview Updates
// ============================================================================

/// Marker for minimap preview nodes that have been populated with an image.
#[derive(Component)]
pub struct MinimapImageAdded;

/// System to update minimap preview nodes with rendered images from the cache.
pub fn update_minimap_previews(
    mut commands: Commands,
    minimap_cache: Res<MinimapCache>,
    preview_query: Query<(Entity, &LevelMiniMapPreview), Without<MinimapImageAdded>>,
) {
    for (entity, preview) in preview_query.iter() {
        if let Some(image_handle) = minimap_cache.images.get(&preview.0) {
            // Add the image as a child of the preview container
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    ImageNode {
                        image: image_handle.clone(),
                        ..default()
                    },
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                ));
            });
            // Mark as populated to avoid re-adding
            commands.entity(entity).insert(MinimapImageAdded);
        }
    }
}
