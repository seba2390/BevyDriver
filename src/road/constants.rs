use bevy::prelude::*;

pub const ROAD_WIDTH: f32 = 50.0;
pub const ROAD_SEGMENT_LENGTH: f32 = ROAD_WIDTH; // Square segments makes ALOT of things easier
pub const STARTING_LINE_WIDTH: f32 = ROAD_WIDTH;
pub const STARTING_LINE_HEIGHT: f32 = 2.0;

// -- Road Edge Settings -- //
/// Width of the glowing road edges
pub const ROAD_EDGE_WIDTH: f32 = 3.0;
/// Z-index for road edges (above road segments)
pub const ROAD_EDGE_Z: f32 = 1.2;

// -- Road Colors -- //
/// Base road color (always this color, no glow)
pub const ROAD_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
/// Emissive color for visited road edges (values > 1.0 for bloom glow)
pub const VISITED_EDGE_COLOR: Color = Color::srgb(1.0, 1.0, 1.6);
/// Color for unvisited road edges (no glow)
pub const UNVISITED_EDGE_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

/// Z-index for the starting line (above road segments)
pub const STARTING_LINE_Z: f32 = 1.5;

/// Z-index for straight road segments
pub const STRAIGHT_ROAD_Z: f32 = 1.0;

/// Z-index for corner road segments
pub const CORNER_ROAD_Z: f32 = 0.0;
