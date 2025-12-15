use bevy::prelude::*;

pub const PROP_Z: f32 = 2.0;

// -- NOS Powerup -- //
pub const NOS_SIZE: f32 = 10.0;
/// Half the size of the NOS powerup (for collision detection)
pub const NOS_HALF_SIZE: f32 = NOS_SIZE / 2.0;
pub const NOS_THICKNESS: f32 = 2.0;
pub const NOS_COLOR: Color = Color::srgb(0.0, 2.0, 2.0); // Cyan glow
pub const NOS_ROTATION_SPEED: f32 = 3.0;
