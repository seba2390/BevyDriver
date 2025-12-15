use bevy::prelude::*;
use crate::car::components::{Car, NosBoostAvailable};
use crate::car::constants::{CAR_WIDTH, CAR_HEIGHT, NOS_AVAILABILITY_DURATION};
use crate::collision::check_obb_collision;
use crate::props::components::NosPowerUp;
use crate::props::constants::*;
use crate::start_menu::components::GameEntity;

/// Half dimensions for car collision box
const CAR_HALF_WIDTH: f32 = CAR_WIDTH / 2.0;
const CAR_HALF_HEIGHT: f32 = CAR_HEIGHT / 2.0;

/// System to check collision between car and NOS powerups using OBB collision detection.
/// Properly handles rotated rectangles for accurate collision.
/// On collision, adds NosBoostAvailable component to the car (resets timer if already present).
pub fn check_powerup_collision(
    mut commands: Commands,
    car_query: Query<(Entity, &Transform), With<Car>>,
    powerup_query: Query<(Entity, &Transform), With<NosPowerUp>>,
) {
    let Some((car_entity, car_transform)) = car_query.iter().next() else {
        return;
    };

    for (powerup_entity, powerup_transform) in &powerup_query {
        // Use OBB collision for accurate rotated rectangle detection
        if check_obb_collision(
            car_transform,
            CAR_HALF_WIDTH,
            CAR_HALF_HEIGHT,
            powerup_transform,
            NOS_HALF_SIZE,
            NOS_HALF_SIZE,
        ) {
            // Despawn the powerup (and its children)
            commands.entity(powerup_entity).despawn();

            // Add or reset NosBoostAvailable on the car
            // This gives the player a window to activate the boost with SPACE
            commands.entity(car_entity).insert(NosBoostAvailable::new(NOS_AVAILABILITY_DURATION));
        }
    }
}

pub fn rotate_powerups(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<NosPowerUp>>,
) {
    for mut transform in &mut query {
        transform.rotate_z(NOS_ROTATION_SPEED * time.delta_secs());
    }
}

pub fn spawn_nos_powerup(commands: &mut Commands, position: Vec2) {
    commands.spawn((
        NosPowerUp,
        Transform::from_xyz(position.x, position.y, PROP_Z),
        Visibility::default(),
        GameEntity,
    )).with_children(|parent| {
        // Top
        parent.spawn((
            Sprite {
                color: NOS_COLOR,
                custom_size: Some(Vec2::new(NOS_SIZE, NOS_THICKNESS)),
                ..default()
            },
            Transform::from_xyz(0.0, NOS_SIZE / 2.0, 0.0),
        ));
        // Bottom
        parent.spawn((
            Sprite {
                color: NOS_COLOR,
                custom_size: Some(Vec2::new(NOS_SIZE, NOS_THICKNESS)),
                ..default()
            },
            Transform::from_xyz(0.0, -NOS_SIZE / 2.0, 0.0),
        ));
        // Left
        parent.spawn((
            Sprite {
                color: NOS_COLOR,
                custom_size: Some(Vec2::new(NOS_THICKNESS, NOS_SIZE)),
                ..default()
            },
            Transform::from_xyz(-NOS_SIZE / 2.0, 0.0, 0.0),
        ));
        // Right
        parent.spawn((
            Sprite {
                color: NOS_COLOR,
                custom_size: Some(Vec2::new(NOS_THICKNESS, NOS_SIZE)),
                ..default()
            },
            Transform::from_xyz(NOS_SIZE / 2.0, 0.0, 0.0),
        ));
    });
}
