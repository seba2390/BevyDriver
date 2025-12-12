pub const ROAD_SEGMENT_LENGTH: f32 = 20.0;
pub const ROAD_WIDTH: f32 = 50.0;
pub const STARTING_LINE_WIDTH: f32 = ROAD_WIDTH;
pub const STARTING_LINE_HEIGHT: f32 = 2.0;

/// Multiplier for positioning the start line ahead of the car's starting point
pub const START_LINE_Y_OFFSET_MULTIPLIER: f32 = 1.5;

/// Z-index for the starting line (above road segments)
pub const STARTING_LINE_Z: f32 = 1.5;

/// Z-index for straight road segments
pub const STRAIGHT_ROAD_Z: f32 = 1.0;

/// Z-index for corner road segments
pub const CORNER_ROAD_Z: f32 = 0.0;
