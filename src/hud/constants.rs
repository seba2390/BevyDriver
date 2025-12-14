use bevy::prelude::*;

// ============================================================================
// Race/Timer Constants
// ============================================================================

/// Time multiplier applied when the car is off the road
pub const OFF_ROAD_TIME_MULTIPLIER: f32 = 10.0;

// ============================================================================
// HUD Layout Constants
// ============================================================================

/// Font size for main HUD elements (timer, level, warnings)
pub const HUD_FONT_SIZE: f32 = 40.0;
/// Padding from screen edges for HUD elements
pub const HUD_PADDING: f32 = 10.0;
/// Multiplier font size ratio relative to HUD_FONT_SIZE
pub const MULTIPLIER_FONT_SIZE_RATIO: f32 = 0.7;
/// Spacing between timer and multiplier text
pub const MULTIPLIER_TOP_SPACING: f32 = 5.0;

// ============================================================================
// Controls Hint Constants
// ============================================================================

/// Font size for controls hint text
pub const CONTROLS_HINT_FONT_SIZE: f32 = 18.0;
/// Padding from screen edges for controls hint
pub const CONTROLS_HINT_PADDING: f32 = 15.0;
/// Vertical spacing between control hint lines
pub const CONTROLS_HINT_LINE_HEIGHT: f32 = 24.0;
/// Horizontal offset for text (leaves room for arrow gizmos)
pub const CONTROLS_HINT_TEXT_OFFSET: f32 = 35.0;
/// Initial alpha for controls hint (semi-transparent)
pub const CONTROLS_HINT_ALPHA: f32 = 0.7;
/// RGB color value for controls hint text
pub const CONTROLS_HINT_RGB: (f32, f32, f32) = (0.8, 0.8, 0.8);
/// Semi-transparent color for the controls hint
pub const CONTROLS_HINT_COLOR: Color = Color::srgba(
    CONTROLS_HINT_RGB.0,
    CONTROLS_HINT_RGB.1,
    CONTROLS_HINT_RGB.2,
    CONTROLS_HINT_ALPHA,
);

/// Delay before controls hint starts fading (seconds)
pub const CONTROLS_FADE_DELAY: f32 = 3.0;
/// Duration of the fade animation (seconds)
pub const CONTROLS_FADE_DURATION: f32 = 1.0;

// ============================================================================
// Controls Hint Arrow Gizmo Constants
// ============================================================================

/// Size of the arrow line
pub const ARROW_SIZE: f32 = 10.0;
/// Size of the arrow head
pub const ARROW_HEAD_SIZE: f32 = 4.0;
/// Horizontal offset for arrow gizmos from base position
pub const ARROW_BASE_X_OFFSET: f32 = 15.0;
/// Vertical offset for arrow gizmos from base position
pub const ARROW_BASE_Y_OFFSET: f32 = 10.0;
/// Vertical offset for up/down arrows from line center
pub const ARROW_VERTICAL_OFFSET: f32 = 5.0;
/// Horizontal offset for left/right steer arrows from center
pub const ARROW_STEER_OFFSET: f32 = 6.0;

/// Velocity threshold squared to detect if player has moved
pub const PLAYER_MOVED_VELOCITY_THRESHOLD: f32 = 10.0;

/// Labels for each control hint line
pub const CONTROL_LABELS: [&str; 4] = ["Accelerate", "Steer", "Brake", "ESC  Pause"];
