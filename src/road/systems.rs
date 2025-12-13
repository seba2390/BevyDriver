use bevy::math::primitives::CircularSector;
use bevy::prelude::*;

use crate::car::components::Car;
use crate::car::constants::CAR_HEIGHT;
use crate::car::helpers::get_car_corners;
use crate::start_menu::components::GameEntity;
use crate::road::components::{
    Direction, FinishLine, RoadSegment, RoadSegmentType, StartLine, Track,
};
use crate::road::constants::*;
use crate::road::helpers::{
    get_direction_vector, get_exit_direction, get_position_offset, get_rotation,
    is_point_in_corner_left, is_point_in_corner_right, is_point_in_straight, validate_track_layout,
};

/// Spawns the start line at the given position
pub fn spawn_start_line(commands: &mut Commands, position: Vec2, direction: Direction) {
    let start_line_sprite = Sprite {
        color: Color::srgb(0.2, 0.8, 0.2), // Green
        custom_size: Some(Vec2::new(STARTING_LINE_WIDTH, STARTING_LINE_HEIGHT)),
        ..default()
    };
    // Place the starting line slightly ahead of the starting point
    let start_line_transform = Transform::from_xyz(
        position.x,
        position.y + CAR_HEIGHT * START_LINE_Y_OFFSET_MULTIPLIER,
        STARTING_LINE_Z,
    );

    commands.spawn((
        start_line_sprite,
        start_line_transform,
        StartLine { direction },
        GameEntity,
    ));
}

/// Spawns the finish line at the given position
pub fn spawn_finish_line(commands: &mut Commands, position: Vec2, direction: Direction) {
    let finish_line_sprite = Sprite {
        color: Color::srgb(1.0, 1.0, 1.0), // White (checkered pattern would be ideal but simple for now)
        custom_size: Some(Vec2::new(STARTING_LINE_WIDTH, STARTING_LINE_HEIGHT)),
        ..default()
    };
    let finish_line_transform = Transform::from_xyz(position.x, position.y, STARTING_LINE_Z);

    commands.spawn((
        finish_line_sprite,
        finish_line_transform,
        FinishLine { direction },
        GameEntity,
    ));
}

pub fn spawn_track(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    track: &Track,
) {
    //validate_track_layout(track.layout);

    let mut current_endpoint = Vec2::new(
        track.starting_point.x,
        track.starting_point.y - ROAD_SEGMENT_LENGTH / 2.0,
    );
    let mut current_direction = Direction::Up;

    for &segment_type in track.layout.iter() {
        match segment_type {
            RoadSegmentType::Straight => {
                current_endpoint = spawn_straight_road(
                    commands,
                    current_endpoint,
                    current_direction,
                    segment_type,
                );
            }
            RoadSegmentType::CornerLeft | RoadSegmentType::CornerRight => {
                let (new_endpoint, new_direction) = spawn_corner_road(
                    commands,
                    meshes,
                    materials,
                    current_endpoint,
                    current_direction,
                    segment_type,
                );
                current_endpoint = new_endpoint;
                current_direction = new_direction;
            }
        }
    }
}

fn spawn_straight_road(
    commands: &mut Commands,
    current_endpoint: Vec2,
    current_direction: Direction,
    segment_type: RoadSegmentType,
) -> Vec2 {
    // Calculate the offset for the straight segment
    let offset = get_position_offset(current_direction);
    // The center of the segment is halfway along the offset from the current endpoint
    let center = current_endpoint + offset / 2.0;

    let road_sprite = Sprite {
        color: Color::srgb(0.6, 0.6, 0.6),
        custom_size: Some(Vec2::new(ROAD_WIDTH, ROAD_SEGMENT_LENGTH)),
        ..default()
    };

    let rotation = get_rotation(current_direction);
    let road_transform = Transform::from_xyz(center.x, center.y, STRAIGHT_ROAD_Z)
        .with_rotation(Quat::from_rotation_z(rotation));

    let road_component = RoadSegment {
        segment_type,
        direction: current_direction,
    };

    commands.spawn((road_sprite, road_transform, road_component, GameEntity));

    // Return the new endpoint (end of this segment)
    current_endpoint + offset
}

fn spawn_corner_road(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    current_endpoint: Vec2,
    current_direction: Direction,
    segment_type: RoadSegmentType,
) -> (Vec2, Direction) {
    let exit_direction = get_exit_direction(current_direction, segment_type);
    let exit_vec = get_direction_vector(exit_direction);
    let entry_vec = get_direction_vector(current_direction);

    // The pivot point (center of the circle defining the corner) is offset from the entry point.
    // We move perpendicular to the entry direction (which is the exit direction for a 90 degree turn)
    // by half the road width to find the inner corner pivot.
    let pivot = current_endpoint + exit_vec * (ROAD_WIDTH / 2.0);

    // Create a 90-degree circular sector
    let sector = CircularSector::from_degrees(ROAD_WIDTH, 90.0);

    // Calculate rotation to align the sector correctly.
    // The sector needs to be rotated 45 degrees (PI/4) relative to the entry direction
    // to align its "pie slice" shape with the corner.
    // For a right turn, we add PI/4. For a left turn, we subtract PI/4.
    let rotation_offset = match segment_type {
        RoadSegmentType::CornerRight => std::f32::consts::FRAC_PI_4,
        RoadSegmentType::CornerLeft => -std::f32::consts::FRAC_PI_4,
        _ => 0.0,
    };
    let rotation_angle = get_rotation(current_direction) + rotation_offset;

    commands.spawn((
        Mesh2d(meshes.add(sector)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.3, 0.3)))),
        Transform::from_xyz(pivot.x, pivot.y, CORNER_ROAD_Z)
            .with_rotation(Quat::from_rotation_z(rotation_angle)),
        RoadSegment {
            segment_type,
            direction: current_direction,
        },
        GameEntity,
    ));

    // The new endpoint is calculated from the pivot.
    // We move from the pivot in the direction of the entry vector by half the road width.
    // This effectively traces the other side of the square that bounds the corner.
    let new_endpoint = pivot + entry_vec * (ROAD_WIDTH / 2.0);
    (new_endpoint, exit_direction)
}

pub fn check_car_on_road(
    car_query: Query<&Transform, With<Car>>,
    road_query: Query<(&Transform, &RoadSegment)>,
) -> bool {
    let car_transform = car_query.single().unwrap();
    let car_corners = get_car_corners(car_transform);

    // For each corner of the car, check if it's on ANY road segment
    for corner in car_corners.iter() {
        let mut corner_on_road = false;

        for (road_transform, road_segment) in road_query.iter() {
            // Transform corner to road's local space
            // Inverse transform is needed because transform_point applies the transform.
            // We want to go from World -> Local.
            // road_transform maps Local -> World.
            // So we need road_transform.inverse().transform_point(corner)
            // But wait, Bevy's Transform doesn't have a simple inverse method that returns a Transform.
            // We can use compute_matrix().inverse().transform_point3()

            let local_corner = road_transform
                .compute_affine()
                .inverse()
                .transform_point3(*corner)
                .xy();

            match road_segment.segment_type {
                RoadSegmentType::Straight => {
                    if is_point_in_straight(local_corner) {
                        corner_on_road = true;
                        break;
                    }
                }
                RoadSegmentType::CornerRight => {
                    if is_point_in_corner_right(local_corner) {
                        corner_on_road = true;
                        break;
                    }
                }
                RoadSegmentType::CornerLeft => {
                    if is_point_in_corner_left(local_corner) {
                        corner_on_road = true;
                        break;
                    }
                }
            }
        }

        // If any corner is off the road, the car is off the road
        if !corner_on_road {
            return false;
        }
    }

    // All corners are on the road
    return true;
}
