use bevy::prelude::*;
use crate::car::constants::{CAR_WIDTH, CAR_HEIGHT};
use crate::collision::get_rect_corners;

/// Half dimensions for car collision box
const CAR_HALF_WIDTH: f32 = CAR_WIDTH / 2.0;
const CAR_HALF_HEIGHT: f32 = CAR_HEIGHT / 2.0;

/// Get the four corners of the car in world space based on its transform.
/// Returns corners in order: front-left, front-right, back-right, back-left
///
/// This is a convenience wrapper around the generic `get_rect_corners` from
/// the collision module, using the car's specific dimensions.
pub fn get_car_corners(transform: &Transform) -> [Vec3; 4] {
    get_rect_corners(transform, CAR_HALF_WIDTH, CAR_HALF_HEIGHT)
}
