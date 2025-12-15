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
/// Horizontal offset for text (leaves room for arrow gizmos, aligned with "SPACE  " width)
pub const CONTROLS_HINT_TEXT_OFFSET: f32 = 76.0;
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

/// Labels for each control hint line (key on left, action on right)
pub const CONTROL_LABELS: [&str; 5] = [
    "Accelerate",
    "Steer",
    "Brake",
    "ESC    Pause",
    "SPACE  Powerup",
];

// ============================================================================
// NOS Boost Bar Constants
// ============================================================================

/// Width of the NOS boost bar
pub const NOS_BAR_WIDTH: f32 = 300.0;
/// Height of the NOS boost bar
pub const NOS_BAR_HEIGHT: f32 = 24.0;
/// Distance from top of screen (calculated to center-align with timer)
/// Timer center Y = HUD_PADDING + HUD_FONT_SIZE/2
/// NOS bar center Y = NOS_BAR_TOP + NOS_BAR_HEIGHT/2
/// To align: NOS_BAR_TOP = HUD_PADDING + HUD_FONT_SIZE/2 - NOS_BAR_HEIGHT/2
pub const NOS_BAR_TOP: f32 = HUD_PADDING + HUD_FONT_SIZE / 2.0 - NOS_BAR_HEIGHT / 2.0;
/// Border width for the bar container (UI element, non-glowing)
pub const NOS_BAR_BORDER: f32 = 2.0;
/// Background color for the bar container (dark)
pub const NOS_BAR_BG_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 0.8);
/// Fill color for the bar (white)
pub const NOS_BAR_FILL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
/// Border color for the bar (non-glowing UI border - dark to not interfere with glow)
pub const NOS_BAR_BORDER_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

// -- NOS Boost Bar Glow (Sprite-based for bloom effect) -- //
/// Glow edge thickness (matches powerup thickness)
pub const NOS_BAR_GLOW_THICKNESS: f32 = 2.0;
/// Glow edge color (HDR cyan matching powerup: values > 1.0 for bloom)
pub const NOS_BAR_GLOW_COLOR: Color = Color::srgb(0.0, 1.7, 1.7);
/// Z-index for glow sprites (above most game elements)
pub const NOS_BAR_GLOW_Z: f32 = 100.0;
