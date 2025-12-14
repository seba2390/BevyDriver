use bevy::prelude::*;

use crate::styles::colors::*;

// ============================================================================
// Shared Button System
// ============================================================================

/// Optional component to override default button colors.
/// Add this to buttons that need custom hover/press colors (e.g., delete buttons).
#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl ButtonColors {
    pub fn new(normal: Color, hovered: Color, pressed: Color) -> Self {
        Self { normal, hovered, pressed }
    }
}

/// Standard button interaction system - handles hover and press visual feedback.
/// Supports optional ButtonColors component for custom colors per button.
/// Use this for all menus instead of duplicating the system per module.
pub fn standard_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&ButtonColors>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, custom_colors) in &mut interaction_query {
        let (normal, hovered, pressed) = if let Some(colors) = custom_colors {
            (colors.normal, colors.hovered, colors.pressed)
        } else {
            (BUTTON_NORMAL_COLOR, BUTTON_HOVERED_COLOR, BUTTON_PRESSED_COLOR)
        };

        *background_color = match *interaction {
            Interaction::Pressed => pressed.into(),
            Interaction::Hovered => hovered.into(),
            Interaction::None => normal.into(),
        };
    }
}

// ============================================================================
// Layout Constants
// ============================================================================

pub const STANDARD_BUTTON_WIDTH: f32 = 250.0;
pub const LARGE_BUTTON_WIDTH: f32 = 300.0;
pub const BUTTON_HEIGHT: f32 = 65.0;
pub const BUTTON_MARGIN: f32 = 20.0;
pub const TITLE_FONT_SIZE: f32 = 60.0;
pub const TITLE_MARGIN_BOTTOM: f32 = 50.0;
pub const BUTTON_FONT_SIZE: f32 = 33.0;
pub const PANEL_PADDING: f32 = 50.0;

// ============================================================================
// Shared Font Sizes
// ============================================================================

/// Large secondary text (time displays, subtitles)
pub const LARGE_TEXT_FONT_SIZE: f32 = 36.0;
/// Medium secondary text (messages, labels)
pub const MEDIUM_TEXT_FONT_SIZE: f32 = 24.0;
/// Small text (stats, details)
pub const SMALL_TEXT_FONT_SIZE: f32 = 18.0;
/// Extra small text (fine print)
pub const XSMALL_TEXT_FONT_SIZE: f32 = 16.0;

// ============================================================================
// Shared Margins/Spacing
// ============================================================================

/// Standard margin between elements
pub const STANDARD_MARGIN: f32 = 20.0;
/// Small margin for tighter spacing
pub const SMALL_MARGIN: f32 = 10.0;
/// Large margin for section separation
pub const LARGE_MARGIN: f32 = 40.0;

// ============================================================================
// Layout Builders
// ============================================================================

/// Full-screen container centered both horizontally and vertically
pub fn fullscreen_centered() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

/// Vertical column layout with centered children
pub fn column_centered() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(PANEL_PADDING)),
        ..default()
    }
}

/// Standard menu button layout
pub fn button_node(button_width: f32) -> Node {
    Node {
        width: Val::Px(button_width),
        height: Val::Px(BUTTON_HEIGHT),
        margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}


// ============================================================================
// Text Style Builders
// ============================================================================

/// Title text style
pub fn title_style() -> (TextFont, TextColor, Node) {
    (
        TextFont {
            font_size: TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(MENU_TEXT_COLOR),
        Node {
            margin: UiRect::bottom(Val::Px(TITLE_MARGIN_BOTTOM)),
            ..default()
        },
    )
}

/// Button text style
pub fn button_text_style() -> (TextFont, TextColor) {
    (
        TextFont {
            font_size: BUTTON_FONT_SIZE,
            ..default()
        },
        TextColor(MENU_TEXT_COLOR),
    )
}

// ============================================================================
// Shared UI Components
// ============================================================================

/// Spawns a "no saves" message with standard styling.
/// Returns the bundle to spawn - caller should add any additional components (like marker components).
pub fn no_saves_message_bundle() -> impl Bundle {
    (
        Text::new("No saved games found.\nStart a new game first!"),
        TextFont {
            font_size: MEDIUM_TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(SECONDARY_TEXT_COLOR),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            margin: UiRect::vertical(Val::Px(LARGE_MARGIN)),
            ..default()
        },
    )
}

// ============================================================================
// Generic Menu Builders
// ============================================================================

/// Spawns a full-screen menu container with the given screen marker and background color.
/// Returns the entity builder so children can be added.
///
/// # Example
/// ```ignore
/// spawn_menu_container(&mut commands, OnMenuScreen, MENU_BACKGROUND_COLOR)
///     .with_children(|parent| {
///         // Add menu content
///     });
/// ```
pub fn spawn_menu_container<'a, M: Component>(
    commands: &'a mut Commands,
    screen_marker: M,
    background_color: Color,
) -> EntityCommands<'a> {
    commands.spawn((
        fullscreen_centered(),
        BackgroundColor(background_color),
        screen_marker,
    ))
}

/// Spawns a standard menu button with the given label and action.
/// Uses STANDARD_BUTTON_WIDTH by default.
pub fn spawn_standard_button<A: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: A,
) {
    spawn_button_with_width(parent, label, action, STANDARD_BUTTON_WIDTH);
}

/// Spawns a menu button with custom width.
pub fn spawn_button_with_width<A: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: A,
    width: f32,
) {
    parent
        .spawn((
            Button,
            button_node(width),
            BackgroundColor(BUTTON_NORMAL_COLOR),
            action,
        ))
        .with_children(|button| {
            button.spawn((Text::new(label), button_text_style()));
        });
}
