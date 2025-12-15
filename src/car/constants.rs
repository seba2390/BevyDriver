pub const CAR_ACCELERATION: f32 = 1000.0;
pub const CAR_MAX_SPEED: f32 = 250.0;
pub const CAR_TURN_SPEED: f32 = 3.0; // Radians per second
pub const CAR_FRICTION: f32 = 400.0; // Rolling resistance
pub const CAR_WIDTH: f32 = 10.0;
pub const CAR_HEIGHT: f32 = 18.0;
pub const CAR_TURN_FACTOR: f32 = 2.0; // Higher means more responsive turning

/// Z-index for the car (above road and starting line)
pub const CAR_Z: f32 = 2.0;

// ============================================================================
// NOS Boost Settings
// ============================================================================

/// Duration (seconds) the NOS boost is available after collecting powerup
pub const NOS_AVAILABILITY_DURATION: f32 = 2.0;
/// Speed multiplier when NOS boost is active
pub const NOS_BOOST_MULTIPLIER: f32 = 1.5;
/// Maximum speed while boosting
pub const NOS_BOOSTED_MAX_SPEED: f32 = CAR_MAX_SPEED * NOS_BOOST_MULTIPLIER;
