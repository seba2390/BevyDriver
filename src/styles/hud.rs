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

/// Off-road warning text style (top-left corner)
pub fn off_road_warning_style() -> (TextFont, TextColor, TextLayout, Node) {
    (
        TextFont {
            font_size: HUD_FONT_SIZE,
            ..default()
        },
        TextColor(WARNING_TEXT),
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
        TextColor(TIMER_RACING),
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
        RaceStatus::WaitingToStart => TextColor(TIMER_WAITING),
        RaceStatus::Racing => TextColor(TIMER_RACING),
        RaceStatus::Finished => TextColor(TIMER_FINISHED),
    }
}
