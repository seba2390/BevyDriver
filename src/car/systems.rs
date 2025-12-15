use crate::car::components::{Car, NosBoostAvailable, Velocity};
use crate::car::constants::*;
use crate::constants::{BOTTOM_BOUNDARY, LEFT_BOUNDARY, RIGHT_BOUNDARY, TOP_BOUNDARY};
use crate::start_menu::components::GameEntity;
use bevy::prelude::*;

// ============================================================================
// Constants for physics tuning
// ============================================================================

/// Drift factor: 0.0 = full grip (on rails), 1.0 = no grip (ice)
const LATERAL_GRIP: f32 = 0.1;

// ============================================================================
// Spawning
// ============================================================================

pub fn spawn_car(commands: &mut Commands, starting_point: Vec2) {
    let car_sprite = Sprite {
        color: Color::srgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(CAR_WIDTH, CAR_HEIGHT)),
        ..default()
    };
    // Place the car slightly behind the starting line
    let car_initial_position = Transform::from_xyz(starting_point.x, starting_point.y, CAR_Z);
    let car_initial_velocity = Velocity(Vec2::ZERO);
    let car_component = Car;

    commands.spawn((
        car_sprite,
        car_initial_position,
        car_initial_velocity,
        car_component,
        GameEntity,
    ));
}

// ============================================================================
// Movement System
// ============================================================================

pub fn move_car(
    mut query: Query<(&mut Transform, &mut Velocity, Option<&NosBoostAvailable>), With<Car>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut transform, mut velocity, boost) in query.iter_mut() {
        apply_lateral_friction(&transform, &mut velocity);
        apply_rolling_friction(&mut velocity, delta);
        clamp_speed(&mut velocity, boost);
        update_position(&mut transform, &velocity, delta);
        clamp_position(&mut transform, &mut velocity);
    }
}

/// Reduces sideways velocity to prevent the car from sliding like on ice.
/// Projects velocity onto forward/right vectors and dampens the lateral component.
fn apply_lateral_friction(transform: &Transform, velocity: &mut Velocity) {
    let forward = (transform.rotation * Vec3::Y).xy();
    let right = (transform.rotation * Vec3::X).xy();

    let forward_velocity = velocity.0.dot(forward);
    let lateral_velocity = velocity.0.dot(right);

    velocity.0 = forward * forward_velocity + right * lateral_velocity * LATERAL_GRIP;
}

/// Applies rolling resistance that slows the car over time.
fn apply_rolling_friction(velocity: &mut Velocity, delta: f32) {
    let speed = velocity.0.length();
    if speed <= 0.0 {
        return;
    }

    let friction_magnitude = CAR_FRICTION * delta;
    if speed < friction_magnitude {
        velocity.0 = Vec2::ZERO;
    } else {
        velocity.0 -= velocity.0.normalize() * friction_magnitude;
    }
}

/// Clamps the car's speed to the maximum allowed.
/// Uses boosted max speed if NOS boost is active.
fn clamp_speed(velocity: &mut Velocity, boost: Option<&NosBoostAvailable>) {
    let max_speed = match boost {
        Some(b) if b.active => NOS_BOOSTED_MAX_SPEED,
        _ => CAR_MAX_SPEED,
    };
    velocity.0 = velocity.0.clamp_length_max(max_speed);
}

/// Updates the car's position based on its current velocity.
fn update_position(transform: &mut Transform, velocity: &Velocity, delta: f32) {
    transform.translation.x += velocity.0.x * delta;
    transform.translation.y += velocity.0.y * delta;
}

/// Clamps the car's position to stay within screen boundaries.
/// Accounts for car rotation when calculating bounds.
/// Zeroes the velocity component when hitting a wall.
fn clamp_position(transform: &mut Transform, velocity: &mut Velocity) {
    // Calculate the rotated bounding box extents
    let (extent_x, extent_y) = get_rotated_extents(transform);

    // Left boundary
    if transform.translation.x - extent_x < LEFT_BOUNDARY {
        transform.translation.x = LEFT_BOUNDARY + extent_x;
        velocity.0.x = velocity.0.x.max(0.0);
    }
    // Right boundary
    if transform.translation.x + extent_x > RIGHT_BOUNDARY {
        transform.translation.x = RIGHT_BOUNDARY - extent_x;
        velocity.0.x = velocity.0.x.min(0.0);
    }
    // Bottom boundary
    if transform.translation.y - extent_y < BOTTOM_BOUNDARY {
        transform.translation.y = BOTTOM_BOUNDARY + extent_y;
        velocity.0.y = velocity.0.y.max(0.0);
    }
    // Top boundary
    if transform.translation.y + extent_y > TOP_BOUNDARY {
        transform.translation.y = TOP_BOUNDARY - extent_y;
        velocity.0.y = velocity.0.y.min(0.0);
    }
}

/// Calculates the axis-aligned bounding box extents for a rotated rectangle.
/// Returns (extent_x, extent_y) - the half-widths in each axis direction.
fn get_rotated_extents(transform: &Transform) -> (f32, f32) {
    let half_width = CAR_WIDTH / 2.0;
    let half_height = CAR_HEIGHT / 2.0;

    // Get the rotation angle from the transform
    let (_, _, angle) = transform.rotation.to_euler(EulerRot::XYZ);

    let cos_a = angle.cos().abs();
    let sin_a = angle.sin().abs();

    // The extents of the AABB for a rotated rectangle
    let extent_x = half_width * cos_a + half_height * sin_a;
    let extent_y = half_width * sin_a + half_height * cos_a;

    (extent_x, extent_y)
}

// ============================================================================
// Input Handling System
// ============================================================================

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Car>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = match query.single_mut() {
        Ok(result) => result,
        Err(_) => return,
    };

    let delta = time.delta_secs();

    apply_steering(&keyboard, &mut transform, delta);
    apply_acceleration(&keyboard, &transform, &mut velocity, delta);
}

/// Handles left/right steering input and rotates the car accordingly.
fn apply_steering(keyboard: &ButtonInput<KeyCode>, transform: &mut Transform, delta: f32) {
    let rotation_input = get_steering_input(keyboard);
    if rotation_input != 0.0 {
        let rotation_amount = rotation_input * CAR_TURN_SPEED * delta;
        transform.rotate_z(rotation_amount);
    }
}

/// Returns the steering input as a value: positive for left, negative for right.
fn get_steering_input(keyboard: &ButtonInput<KeyCode>) -> f32 {
    let mut input = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) {
        input += CAR_TURN_FACTOR;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        input -= CAR_TURN_FACTOR;
    }
    input
}

/// Handles up/down acceleration input and applies force in the car's facing direction.
fn apply_acceleration(
    keyboard: &ButtonInput<KeyCode>,
    transform: &Transform,
    velocity: &mut Velocity,
    delta: f32,
) {
    let acceleration_input = get_acceleration_input(keyboard);
    if acceleration_input != 0.0 {
        let forward_direction = (transform.rotation * Vec3::Y).xy();
        velocity.0 += forward_direction * acceleration_input * delta;
    }
}

/// Returns the acceleration input: positive for forward, negative for reverse.
fn get_acceleration_input(keyboard: &ButtonInput<KeyCode>) -> f32 {
    let mut input = 0.0;
    if keyboard.pressed(KeyCode::ArrowUp) {
        input += CAR_ACCELERATION;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        input -= CAR_ACCELERATION;
    }
    input
}

// ============================================================================
// NOS Boost System
// ============================================================================

/// System to update NOS boost availability timer and handle SPACE activation.
/// - Ticks the availability timer (boost window counting down)
/// - Sets active=true while SPACE is held (if boost is available)
/// - Removes NosBoostAvailable component when timer expires
pub fn update_nos_boost(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut NosBoostAvailable), With<Car>>,
) {
    for (entity, mut boost) in query.iter_mut() {
        // Tick the availability timer
        boost.timer.tick(time.delta());

        // Activate boost while SPACE is held (only if timer hasn't expired)
        boost.active = keyboard.pressed(KeyCode::Space) && !boost.timer.is_finished();

        // Remove component when availability window expires
        if boost.timer.is_finished() {
            commands.entity(entity).remove::<NosBoostAvailable>();
        }

        // TODO: Spawn particle trail while boost.active is true
    }
}
