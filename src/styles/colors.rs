use bevy::prelude::*;

// ============================================================================
// Menu Colors
// ============================================================================

pub const MENU_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const MENU_BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.2);
pub const OVERLAY_BACKGROUND_COLOR: Color = Color::srgba(0.1, 0.1, 0.15, 0.85);
pub const BUTTON_NORMAL_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BUTTON_HOVERED_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.35, 0.35, 0.35);

// ============================================================================
// Shared Text Colors
// ============================================================================

/// Secondary/muted text color for stats, placeholders, etc.
pub const SECONDARY_TEXT_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
/// Success/positive text color (green)
pub const SUCCESS_TEXT_COLOR: Color = Color::srgb(0.3, 1.0, 0.3);
/// Error text color (red)
pub const ERROR_TEXT_COLOR: Color = Color::srgb(1.0, 0.3, 0.3);

// ============================================================================
// HUD Colors
// ============================================================================

pub const WARNING_TEXT_COLOR: Color = Color::srgb(1.0, 0.2, 0.2);
pub const TIMER_WAITING_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
pub const LEVEL_TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const TIMER_RACING_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const TIMER_FINISHED_COLOR: Color = Color::srgb(0.2, 1.0, 0.2);
