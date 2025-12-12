use bevy::prelude::*;
use crate::car::constants::{CAR_WIDTH, CAR_HEIGHT};

/// Get the four corners of the car in world space based on its transform.
/// Returns corners in order: front-left, front-right, back-right, back-left
pub fn get_car_corners(transform: &Transform) -> [Vec3; 4] {
    let half_w = CAR_WIDTH / 2.0;
    let half_h = CAR_HEIGHT / 2.0;

    // Local space corners (relative to car center)
    let local_corners = [
        Vec3::new(-half_w, half_h, 0.0),  // front-left
        Vec3::new(half_w, half_h, 0.0),   // front-right
        Vec3::new(half_w, -half_h, 0.0),  // back-right
        Vec3::new(-half_w, -half_h, 0.0), // back-left
    ];

    // Transform each corner to world space
    local_corners.map(|corner| transform.transform_point(corner))
}
