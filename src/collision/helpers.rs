//! Collision detection helpers for rotated rectangles (OBB collision).
//!
//! This module provides generic utilities for detecting collisions between
//! oriented bounding boxes (rotated rectangles), used for car-powerup collisions
//! and other game objects.

use bevy::prelude::*;

/// Get the four corners of a rectangle in world space based on its transform.
/// Returns corners in order: front-left, front-right, back-right, back-left
///
/// # Arguments
/// * `transform` - The transform of the rectangle (position, rotation, scale)
/// * `half_width` - Half the width of the rectangle (X axis extent)
/// * `half_height` - Half the height of the rectangle (Y axis extent)
pub fn get_rect_corners(transform: &Transform, half_width: f32, half_height: f32) -> [Vec3; 4] {
    // Local space corners (relative to rectangle center)
    let local_corners = [
        Vec3::new(-half_width, half_height, 0.0),  // front-left
        Vec3::new(half_width, half_height, 0.0),   // front-right
        Vec3::new(half_width, -half_height, 0.0),  // back-right
        Vec3::new(-half_width, -half_height, 0.0), // back-left
    ];

    // Transform each corner to world space
    local_corners.map(|corner| transform.transform_point(corner))
}

/// Transform a world-space point to local 2D coordinates relative to a transform.
/// This is commonly used for checking if points are inside rotated shapes.
pub fn world_to_local_2d(transform: &Transform, world_point: Vec3) -> Vec2 {
    transform
        .compute_affine()
        .inverse()
        .transform_point3(world_point)
        .xy()
}

/// Check if a point (in local space) is inside an axis-aligned rectangle.
/// The rectangle is centered at (0,0) with the given half-dimensions.
///
/// # Arguments
/// * `local_pos` - The point position in local space
/// * `half_width` - Half the width of the rectangle (X axis extent)
/// * `half_height` - Half the height of the rectangle (Y axis extent)
pub fn is_point_in_rect(local_pos: Vec2, half_width: f32, half_height: f32) -> bool {
    local_pos.x.abs() <= half_width && local_pos.y.abs() <= half_height
}

/// Check collision between two oriented bounding boxes (rotated rectangles).
/// Uses a point-in-OBB approach: checks if any corner of rectangle A is inside
/// rectangle B, and vice versa.
///
/// This handles most practical collision cases. For edge cases where rectangles
/// overlap without any corners being inside (e.g., thin rectangles crossing at
/// angles), consider using full Separating Axis Theorem (SAT) instead.
///
/// # Arguments
/// * `transform_a` - Transform of the first rectangle
/// * `half_width_a` - Half width of the first rectangle
/// * `half_height_a` - Half height of the first rectangle
/// * `transform_b` - Transform of the second rectangle
/// * `half_width_b` - Half width of the second rectangle
/// * `half_height_b` - Half height of the second rectangle
pub fn check_obb_collision(
    transform_a: &Transform,
    half_width_a: f32,
    half_height_a: f32,
    transform_b: &Transform,
    half_width_b: f32,
    half_height_b: f32,
) -> bool {
    // Get corners of both rectangles in world space
    let corners_a = get_rect_corners(transform_a, half_width_a, half_height_a);
    let corners_b = get_rect_corners(transform_b, half_width_b, half_height_b);

    // Check if any corner of A is inside B
    for corner in &corners_a {
        let local = world_to_local_2d(transform_b, *corner);
        if is_point_in_rect(local, half_width_b, half_height_b) {
            return true;
        }
    }

    // Check if any corner of B is inside A
    for corner in &corners_b {
        let local = world_to_local_2d(transform_a, *corner);
        if is_point_in_rect(local, half_width_a, half_height_a) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_4;

    #[test]
    fn test_no_collision_separated() {
        let transform_a = Transform::from_xyz(0.0, 0.0, 0.0);
        let transform_b = Transform::from_xyz(100.0, 0.0, 0.0);

        assert!(!check_obb_collision(
            &transform_a, 5.0, 9.0,
            &transform_b, 5.0, 5.0,
        ));
    }

    #[test]
    fn test_collision_overlapping() {
        let transform_a = Transform::from_xyz(0.0, 0.0, 0.0);
        let transform_b = Transform::from_xyz(5.0, 0.0, 0.0);

        assert!(check_obb_collision(
            &transform_a, 5.0, 9.0,
            &transform_b, 5.0, 5.0,
        ));
    }

    #[test]
    fn test_collision_rotated() {
        let transform_a = Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(FRAC_PI_4));
        let transform_b = Transform::from_xyz(8.0, 0.0, 0.0);

        // Rotated 45° rectangle with half_height=9 should extend further on X axis
        assert!(check_obb_collision(
            &transform_a, 5.0, 9.0,
            &transform_b, 5.0, 5.0,
        ));
    }
}
