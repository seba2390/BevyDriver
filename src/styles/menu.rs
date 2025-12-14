use bevy::prelude::*;

use crate::styles::colors::*;

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
