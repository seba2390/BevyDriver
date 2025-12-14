use bevy::prelude::*;

// ============================================================================
// Input Constraints
// ============================================================================

pub const MAX_NAME_LENGTH: usize = 20;

// ============================================================================
// Layout Constants
// ============================================================================

pub const INPUT_FIELD_WIDTH: f32 = 400.0;
pub const INPUT_FIELD_HEIGHT: f32 = 50.0;
pub const INPUT_FIELD_PADDING: f32 = 15.0;
pub const INPUT_FIELD_BORDER_WIDTH: f32 = 2.0;
pub const INPUT_FIELD_MARGIN: f32 = 10.0;
pub const SUBTITLE_MARGIN: f32 = 20.0;
pub const ERROR_TEXT_HEIGHT: f32 = 25.0;
pub const ERROR_TEXT_MARGIN: f32 = 10.0;

// ============================================================================
// Font Sizes
// ============================================================================

pub const SUBTITLE_FONT_SIZE: f32 = 24.0;
pub const ERROR_TEXT_FONT_SIZE: f32 = 18.0;

// ============================================================================
// Colors
// ============================================================================

pub const INPUT_BACKGROUND_COLOR: Color = Color::srgb(0.2, 0.2, 0.25);
pub const INPUT_BORDER_COLOR: Color = Color::srgb(0.4, 0.4, 0.5);
pub const PLACEHOLDER_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const ERROR_TEXT_COLOR: Color = Color::srgb(1.0, 0.3, 0.3);
