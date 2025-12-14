use bevy::prelude::*;

use crate::hud::constants::{
    CONTROLS_HINT_COLOR, CONTROLS_HINT_FONT_SIZE, CONTROLS_HINT_LINE_HEIGHT,
    CONTROLS_HINT_PADDING, CONTROLS_HINT_TEXT_OFFSET, HUD_FONT_SIZE, HUD_PADDING,
    MULTIPLIER_FONT_SIZE_RATIO, MULTIPLIER_TOP_SPACING,
};
use crate::styles::colors::*;

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

/// Multiplier indicator text style (below timer, top-right corner)
pub fn multiplier_style() -> (TextFont, TextColor, TextLayout, Node) {
    (
        TextFont {
            font_size: HUD_FONT_SIZE * MULTIPLIER_FONT_SIZE_RATIO,
            ..default()
        },
        TextColor(WARNING_TEXT_COLOR),
        TextLayout::new_with_justify(Justify::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(HUD_PADDING + HUD_FONT_SIZE + MULTIPLIER_TOP_SPACING),
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

// ============================================================================
// Controls Hint Style
// ============================================================================

/// Controls hint text style for a specific line (bottom-left corner)
/// line_index: 0 = top line (Accelerate), 1 = Steer, 2 = Brake, 3 = ESC Pause
pub fn controls_hint_line_style(line_index: usize) -> (TextFont, TextColor, TextLayout, Node) {
    // Calculate bottom offset: line 0 is at top, so higher bottom value
    let bottom_offset =
        CONTROLS_HINT_PADDING + (3 - line_index) as f32 * CONTROLS_HINT_LINE_HEIGHT;
    // Left offset to leave room for arrow gizmos (ESC line starts further left since no arrow)
    let left_offset = if line_index == 3 {
        CONTROLS_HINT_PADDING // ESC line - no arrow, so align with edge
    } else {
        CONTROLS_HINT_PADDING + CONTROLS_HINT_TEXT_OFFSET // Arrow lines - offset for gizmo space
    };

    (
        TextFont {
            font_size: CONTROLS_HINT_FONT_SIZE,
            ..default()
        },
        TextColor(CONTROLS_HINT_COLOR),
        TextLayout::new_with_justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(bottom_offset),
            left: Val::Px(left_offset),
            ..default()
        },
    )
}
