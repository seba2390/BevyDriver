use crate::road::components::Direction;
use crate::road::constants::ROAD_WIDTH;

/// Horizontal tolerance for detecting line crossing (half the road width)
pub const LINE_X_TOLERANCE: f32 = ROAD_WIDTH / 2.0;

/// Check if the car is within the horizontal bounds of a line
pub fn is_within_line_x_bounds(car_x: f32, line_x: f32) -> bool {
    (car_x - line_x).abs() < LINE_X_TOLERANCE
}

/// Check if the car has crossed a line in the specified direction.
/// Returns true if the car was on one side of the line last frame and is now on the other side.
///
/// # Arguments
/// * `car_y` - Current Y position of the car
/// * `last_y` - Y position of the car last frame
/// * `line_y` - Y position of the line
/// * `direction` - The direction the car must be moving to trigger the crossing
pub fn has_crossed_line(car_y: f32, last_y: f32, line_y: f32, direction: Direction) -> bool {
    match direction {
        Direction::Up => last_y <= line_y && car_y > line_y,
        Direction::Down => last_y >= line_y && car_y < line_y,
        // For horizontal movement, we'd check X instead of Y
        // These can be implemented when needed
        Direction::Left | Direction::Right => false,
    }
}

/// Format elapsed time as a string with 2 decimal places
pub fn format_elapsed_time(elapsed_secs: f32) -> String {
    format!("{:.2}", elapsed_secs)
}
