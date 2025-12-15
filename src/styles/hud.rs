use bevy::prelude::*;

use crate::hud::constants::{
    CONTROLS_HINT_COLOR, CONTROLS_HINT_FONT_SIZE, CONTROLS_HINT_LINE_HEIGHT,
    CONTROLS_HINT_PADDING, CONTROLS_HINT_TEXT_OFFSET, HUD_FONT_SIZE, HUD_PADDING,
    MULTIPLIER_FONT_SIZE_RATIO, MULTIPLIER_TOP_SPACING, NOS_BAR_BG_COLOR, NOS_BAR_BORDER,
    NOS_BAR_BORDER_COLOR, NOS_BAR_FILL_COLOR, NOS_BAR_HEIGHT, NOS_BAR_TOP, NOS_BAR_WIDTH,
};
use crate::styles::colors::*;

// ============================================================================
// Style Builders
// ============================================================================

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

/// Number of control hint lines (Accelerate, Steer, Brake, ESC Pause, SPACE Powerup)
const CONTROLS_HINT_LINE_COUNT: usize = 5;

/// Controls hint text style for a specific line (bottom-left corner)
/// line_index: 0 = top line (Accelerate), 1 = Steer, 2 = Brake, 3 = ESC Pause, 4 = SPACE Powerup
pub fn controls_hint_line_style(line_index: usize) -> (TextFont, TextColor, TextLayout, Node) {
    // Calculate bottom offset: line 0 is at top, so higher bottom value
    let bottom_offset =
        CONTROLS_HINT_PADDING + (CONTROLS_HINT_LINE_COUNT - 1 - line_index) as f32 * CONTROLS_HINT_LINE_HEIGHT;
    // Arrow lines start after arrow gizmo space, ESC/SPACE lines start at edge (key replaces arrow)
    let left_offset = if line_index >= 3 {
        CONTROLS_HINT_PADDING // ESC/SPACE lines - key on left where arrows would be
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

// ============================================================================
// NOS Boost Bar Style
// ============================================================================

/// Container node for the NOS boost bar (centered at bottom)
pub fn nos_bar_container_style() -> Node {
    Node {
        position_type: PositionType::Absolute,
        top: Val::Px(NOS_BAR_TOP),
        left: Val::Percent(50.0),
        // Center horizontally by offsetting by half the width
        margin: UiRect::left(Val::Px(-NOS_BAR_WIDTH / 2.0)),
        width: Val::Px(NOS_BAR_WIDTH),
        height: Val::Px(NOS_BAR_HEIGHT),
        border: UiRect::all(Val::Px(NOS_BAR_BORDER)),
        ..default()
    }
}

/// Background color for the NOS bar container
pub fn nos_bar_container_colors() -> (BackgroundColor, BorderColor) {
    (
        BackgroundColor(NOS_BAR_BG_COLOR),
        BorderColor::all(NOS_BAR_BORDER_COLOR),
    )
}

/// Fill node for the NOS boost bar (starts full width, shrinks as time runs out)
pub fn nos_bar_fill_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }
}

/// Fill color for the NOS bar
pub fn nos_bar_fill_color() -> BackgroundColor {
    BackgroundColor(NOS_BAR_FILL_COLOR)
}
