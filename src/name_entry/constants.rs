use bevy::prelude::*;

use crate::styles::menu::{MEDIUM_TEXT_FONT_SIZE, SMALL_MARGIN, SMALL_TEXT_FONT_SIZE, STANDARD_MARGIN};

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
pub const INPUT_FIELD_MARGIN: f32 = SMALL_MARGIN;
pub const SUBTITLE_MARGIN: f32 = STANDARD_MARGIN;
pub const ERROR_TEXT_HEIGHT: f32 = 25.0;
pub const ERROR_TEXT_MARGIN: f32 = SMALL_MARGIN;

// ============================================================================
// Font Sizes
// ============================================================================

pub const SUBTITLE_FONT_SIZE: f32 = MEDIUM_TEXT_FONT_SIZE;
pub const ERROR_TEXT_FONT_SIZE: f32 = SMALL_TEXT_FONT_SIZE;

// ============================================================================
// Colors
// ============================================================================

pub const INPUT_BACKGROUND_COLOR: Color = Color::srgb(0.2, 0.2, 0.25);
pub const INPUT_BORDER_COLOR: Color = Color::srgb(0.4, 0.4, 0.5);
pub const PLACEHOLDER_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
