use bevy::prelude::*;
use crate::road::components::{Direction, RoadSegmentType};
use crate::road::constants::{ROAD_SEGMENT_LENGTH, ROAD_WIDTH};


/// Check if a point (in local space) is inside a straight road segment.
/// The straight road is a rectangle centered at (0,0) with dimensions ROAD_WIDTH x ROAD_SEGMENT_LENGTH.
pub fn is_point_in_straight(local_pos: Vec2) -> bool {
    let half_w = ROAD_WIDTH / 2.0;
    let half_h = ROAD_SEGMENT_LENGTH / 2.0;

    local_pos.x >= -half_w
        && local_pos.x <= half_w
        && local_pos.y >= -half_h
        && local_pos.y <= half_h
}

/// Check if a point (in local space) is inside a right corner segment.
///
/// The CircularSector primitive is created with a 90° arc centered on the +Y axis,
/// meaning it spans from 45° to 135° when measured from the +X axis (using atan2).
///
/// While the corner is rotated by +PI/4 in world space during spawning, the inverse
/// transform in check_car_on_road brings us back to this original local orientation.
/// Therefore, we check if the angle falls within PI/4 to 3*PI/4 (45° to 135°).
pub fn is_point_in_corner_right(local_pos: Vec2) -> bool {
    let distance = local_pos.length();
    if distance > ROAD_WIDTH {
        return false;
    }
    // atan2(y, x) gives angle from +X axis in range [-PI, PI]
    let angle = local_pos.y.atan2(local_pos.x);
    // Sector centered on +Y axis spans PI/4 (45°) to 3*PI/4 (135°)
    (std::f32::consts::FRAC_PI_4..=(std::f32::consts::FRAC_PI_4 + std::f32::consts::FRAC_PI_2))
        .contains(&angle)
}

/// Check if a point (in local space) is inside a left corner segment.
///
/// The CircularSector primitive is created with a 90° arc centered on the +Y axis,
/// meaning it spans from 45° to 135° when measured from the +X axis (using atan2).
///
/// While the corner is rotated by -PI/4 in world space during spawning, the inverse
/// transform in check_car_on_road brings us back to this original local orientation.
/// Therefore, we check if the angle falls within PI/4 to 3*PI/4 (45° to 135°),
/// which is the same as CornerRight since both use the same CircularSector primitive.
pub fn is_point_in_corner_left(local_pos: Vec2) -> bool {
    let distance = local_pos.length();
    if distance > ROAD_WIDTH {
        return false;
    }
    // atan2(y, x) gives angle from +X axis in range [-PI, PI]
    let angle = local_pos.y.atan2(local_pos.x);
    // Sector centered on +Y axis spans PI/4 (45°) to 3*PI/4 (135°)
    ((std::f32::consts::FRAC_PI_2 - std::f32::consts::FRAC_PI_4)
        ..=(std::f32::consts::PI - std::f32::consts::FRAC_PI_4))
        .contains(&angle)
}

/// Given an entry direction and segment type, return the exit direction
pub fn get_exit_direction(entry_direction: Direction, segment_type: RoadSegmentType) -> Direction {
    match segment_type {
        RoadSegmentType::Straight => entry_direction,
        RoadSegmentType::CornerLeft => match entry_direction {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        },
        RoadSegmentType::CornerRight => match entry_direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        },
    }
}

/// Get position offset for the next segment based on current direction
pub fn get_position_offset(direction: Direction) -> Vec2 {
    match direction {
        Direction::Up => Vec2::new(0.0, ROAD_SEGMENT_LENGTH),
        Direction::Down => Vec2::new(0.0, -ROAD_SEGMENT_LENGTH),
        Direction::Left => Vec2::new(-ROAD_SEGMENT_LENGTH, 0.0),
        Direction::Right => Vec2::new(ROAD_SEGMENT_LENGTH, 0.0),
    }
}

/// Get sprite rotation based on direction (in radians)
pub fn get_rotation(direction: Direction) -> f32 {
    match direction {
        Direction::Up => 0.0,
        Direction::Right => -std::f32::consts::FRAC_PI_2, // -90 degrees
        Direction::Down => std::f32::consts::PI,          // 180 degrees
        Direction::Left => std::f32::consts::FRAC_PI_2,   // 90 degrees
    }
}

pub fn get_direction_vector(direction: Direction) -> Vec2 {
    match direction {
        Direction::Up => Vec2::Y,
        Direction::Down => Vec2::NEG_Y,
        Direction::Left => Vec2::NEG_X,
        Direction::Right => Vec2::X,
    }
}

/// Transform a world-space point to local 2D coordinates relative to a transform.
/// This is commonly used for checking if points are inside road segments.
pub fn world_to_local_2d(transform: &Transform, world_point: Vec3) -> Vec2 {
    transform
        .compute_affine()
        .inverse()
        .transform_point3(world_point)
        .xy()
}

/// Check if a point (in local space) is inside a road segment of the given type.
/// Dispatches to the appropriate geometry check based on segment type.
pub fn is_point_in_segment(local_pos: Vec2, segment_type: RoadSegmentType) -> bool {
    match segment_type {
        RoadSegmentType::Straight => is_point_in_straight(local_pos),
        RoadSegmentType::CornerRight => is_point_in_corner_right(local_pos),
        RoadSegmentType::CornerLeft => is_point_in_corner_left(local_pos),
    }
}
