use bevy::prelude::*;

use crate::styles::colors::*;

// ============================================================================
// Layout Constants
// ============================================================================

pub const HUD_FONT_SIZE: f32 = 40.0;
pub const HUD_PADDING: f32 = 10.0;

// ============================================================================
// Style Builders
// ============================================================================

/// Off-road warning text style (top-center)
/// Centered by spanning full width (left=0, right=0, width=100%) and using Justify::Center for text
pub fn off_road_warning_style() -> (TextFont, TextColor, TextLayout, Node) {
    (
        TextFont {
            font_size: HUD_FONT_SIZE,
            ..default()
        },
        TextColor(WARNING_TEXT_COLOR),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(HUD_PADDING),
            left: Val::Px(0.),
            right: Val::Px(0.),
            width: Val::Percent(100.),
            ..default()
        },
    )
}

/// Level text style (top-left corner)
pub fn level_text_style() -> (TextFont, TextColor, TextLayout, Node) {
    (
        TextFont {
            font_size: HUD_FONT_SIZE,
            ..default()
        },
        TextColor(LEVEL_TEXT_COLOR),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(HUD_PADDING),
            left: Val::Px(HUD_PADDING),
            ..default()
        },
    )
}

/// Timer text style (top-right corner)
pub fn timer_style() -> (TextFont, TextColor, TextLayout, Node) {
    (
        TextFont {
            font_size: HUD_FONT_SIZE,
            ..default()
        },
        TextColor(TIMER_RACING_COLOR),
        TextLayout::new_with_justify(Justify::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(HUD_PADDING),
            right: Val::Px(HUD_PADDING),
            ..default()
        },
    )
}

/// Returns the timer color based on race status
pub fn timer_color(status: &crate::hud::components::RaceStatus) -> TextColor {
    use crate::hud::components::RaceStatus;
    match status {
        RaceStatus::WaitingToStart => TextColor(TIMER_WAITING_COLOR),
        RaceStatus::Racing => TextColor(TIMER_RACING_COLOR),
        RaceStatus::Finished => TextColor(TIMER_FINISHED_COLOR),
    }
}
