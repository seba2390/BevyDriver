use bevy::prelude::*;

// ============================================================================
// Layout Constants
// ============================================================================

pub const SAVE_SLOT_WIDTH: f32 = 500.0;
pub const SAVE_SLOT_HEIGHT: f32 = 80.0;
pub const SAVE_SLOT_PADDING: f32 = 15.0;
pub const SAVE_SLOT_SPACING: f32 = 10.0;
pub const DELETE_BUTTON_SIZE: f32 = 40.0;
pub const SCROLL_CONTAINER_HEIGHT: f32 = 400.0;
pub const SAVES_LIST_MARGIN: f32 = 20.0;
pub const NO_SAVES_MESSAGE_MARGIN: f32 = 40.0;
pub const CONFIRM_BUTTON_WIDTH: f32 = 150.0;
pub const CONFIRM_BUTTON_SPACING: f32 = 20.0;

// ============================================================================
// Font Sizes
// ============================================================================

pub const PLAYER_NAME_FONT_SIZE: f32 = 28.0;
pub const STATS_LINE_FONT_SIZE: f32 = 16.0;
pub const NO_SAVES_MESSAGE_FONT_SIZE: f32 = 24.0;
pub const CONFIRM_MESSAGE_FONT_SIZE: f32 = 32.0;
pub const DELETE_BUTTON_FONT_SIZE: f32 = 20.0;

// ============================================================================
// Colors
// ============================================================================

pub const DELETE_BUTTON_COLOR: Color = Color::srgb(0.6, 0.2, 0.2);
pub const DELETE_BUTTON_HOVER: Color = Color::srgb(0.8, 0.3, 0.3);
pub const DELETE_BUTTON_PRESSED: Color = Color::srgb(0.9, 0.2, 0.2);
pub const SECONDARY_TEXT_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
