use bevy::math::primitives::CircularSector;
use bevy::prelude::*;

use crate::car::components::Car;
use crate::car::constants::CAR_HEIGHT;
use crate::car::helpers::get_car_corners;
use crate::start_menu::components::GameEntity;
use crate::road::components::{
    Direction, FinishLine, RoadEdge, RoadSegment, RoadSegmentType, StartLine, Track, Visited,
};
use crate::road::constants::*;
use crate::road::helpers::{
    get_direction_vector, get_exit_direction, get_position_offset, get_rotation,
    is_point_in_segment, world_to_local_2d,
};

/// Spawns the start line at the given position
pub fn spawn_start_line(commands: &mut Commands, position: Vec2, direction: Direction) {
    let start_line_sprite = Sprite {
        color: Color::srgb(0.2, 0.8, 0.2), // Green
        custom_size: Some(Vec2::new(STARTING_LINE_WIDTH, STARTING_LINE_HEIGHT)),
        ..default()
    };
    // Place the starting line CAR_HEIGHT ahead of the starting point
    // This mirrors the finish line which is CAR_HEIGHT behind the starting point
    let start_line_transform = Transform::from_xyz(
        position.x,
        position.y + CAR_HEIGHT,
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
        color: ROAD_SEGMENT_COLOR,
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

    let segment_entity = commands.spawn((road_sprite, road_transform, road_component, GameEntity)).id();

    // Spawn glowing edges on both sides of the road
    spawn_straight_road_edges(commands, center, rotation, segment_entity);

    // Return the new endpoint (end of this segment)
    current_endpoint + offset
}

/// Spawns two edge sprites on either side of a straight road segment
fn spawn_straight_road_edges(
    commands: &mut Commands,
    center: Vec2,
    rotation: f32,
    parent_segment: Entity,
) {
    let rotation_quat = Quat::from_rotation_z(rotation);

    // Calculate perpendicular offset for edge placement (left and right of center)
    // Place edges on the OUTSIDE of the road
    let perpendicular = rotation_quat.mul_vec3(Vec3::X).xy();
    let edge_offset = perpendicular * (ROAD_WIDTH / 2.0 + ROAD_EDGE_WIDTH / 2.0);

    // Left edge
    let left_edge_sprite = Sprite {
        color: UNVISITED_EDGE_COLOR,
        custom_size: Some(Vec2::new(ROAD_EDGE_WIDTH, ROAD_SEGMENT_LENGTH)),
        ..default()
    };
    let left_edge_pos = center - edge_offset;
    commands.spawn((
        left_edge_sprite,
        Transform::from_xyz(left_edge_pos.x, left_edge_pos.y, ROAD_EDGE_Z)
            .with_rotation(rotation_quat),
        RoadEdge { parent_segment },
        GameEntity,
    ));

    // Right edge
    let right_edge_sprite = Sprite {
        color: UNVISITED_EDGE_COLOR,
        custom_size: Some(Vec2::new(ROAD_EDGE_WIDTH, ROAD_SEGMENT_LENGTH)),
        ..default()
    };
    let right_edge_pos = center + edge_offset;
    commands.spawn((
        right_edge_sprite,
        Transform::from_xyz(right_edge_pos.x, right_edge_pos.y, ROAD_EDGE_Z)
            .with_rotation(rotation_quat),
        RoadEdge { parent_segment },
        GameEntity,
    ));
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

    let segment_entity = commands.spawn((
        Mesh2d(meshes.add(sector)),
        MeshMaterial2d(materials.add(ColorMaterial::from(ROAD_SEGMENT_COLOR))),
        Transform::from_xyz(pivot.x, pivot.y, CORNER_ROAD_Z)
            .with_rotation(Quat::from_rotation_z(rotation_angle)),
        RoadSegment {
            segment_type,
            direction: current_direction,
        },
        GameEntity,
    )).id();

    // Spawn glowing arc edges for the corner
    spawn_corner_road_edges(commands, meshes, materials, pivot, rotation_angle, segment_type, segment_entity);

    // The new endpoint is calculated from the pivot.
    // We move from the pivot in the direction of the entry vector by half the road width.
    // This effectively traces the other side of the square that bounds the corner.
    let new_endpoint = pivot + entry_vec * (ROAD_WIDTH / 2.0);
    (new_endpoint, exit_direction)
}

/// Spawns two arc edge meshes for corner road segments (inner and outer arcs)
fn spawn_corner_road_edges(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pivot: Vec2,
    rotation_angle: f32,
    _segment_type: RoadSegmentType,
    parent_segment: Entity,
) {
    // Inner arc edge - placed OUTSIDE the inner edge of the road
    // The road goes from radius 0 to ROAD_WIDTH, so inner edge is a thin arc just inside radius 0
    // We need a small arc from 0 to ROAD_EDGE_WIDTH (but this would be inside, not outside)
    // Actually for "outside", we want edges beyond the road boundaries
    // Inner edge: arc from -ROAD_EDGE_WIDTH to 0 (but can't have negative radius)
    // So we skip the inner edge for corners or use a different approach

    // For corners, "outside" means:
    // - Outer edge: beyond ROAD_WIDTH (from ROAD_WIDTH to ROAD_WIDTH + ROAD_EDGE_WIDTH)
    // - Inner edge: there's no "outside" on the pivot side, so we skip it

    // Outer arc edge - placed OUTSIDE the outer edge of the road
    let outer_radius = ROAD_WIDTH + ROAD_EDGE_WIDTH;
    let outer_inner_radius = ROAD_WIDTH;

    // Create outer arc using a circular sector at outer edge
    let outer_sector = CircularSector::from_degrees(outer_radius, 90.0);
    let cutout_sector = CircularSector::from_degrees(outer_inner_radius, 90.0);

    // Spawn outer edge (full outer sector)
    commands.spawn((
        Mesh2d(meshes.add(outer_sector)),
        MeshMaterial2d(materials.add(ColorMaterial::from(UNVISITED_EDGE_COLOR))),
        Transform::from_xyz(pivot.x, pivot.y, ROAD_EDGE_Z)
            .with_rotation(Quat::from_rotation_z(rotation_angle)),
        RoadEdge { parent_segment },
        GameEntity,
    ));

    // Spawn cutout to create ring effect (same color as background/transparent)
    commands.spawn((
        Mesh2d(meshes.add(cutout_sector)),
        MeshMaterial2d(materials.add(ColorMaterial::from(ROAD_SEGMENT_COLOR))),
        Transform::from_xyz(pivot.x, pivot.y, ROAD_EDGE_Z + 0.01)
            .with_rotation(Quat::from_rotation_z(rotation_angle)),
        GameEntity,
    ));
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
            let local_corner = world_to_local_2d(road_transform, *corner);

            if is_point_in_segment(local_corner, road_segment.segment_type) {
                corner_on_road = true;
                break;
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

/// System to mark road segments as visited when the car touches them and change their color.
/// Uses Without<Visited> filter to only query unvisited segments.
/// A segment is marked visited as soon as any part of the car (any corner) touches it.
pub fn update_segment_visited_status(
    mut commands: Commands,
    car_query: Query<&Transform, With<Car>>,
    mut road_query: Query<
        (
            Entity,
            &Transform,
            &RoadSegment,
        ),
        Without<Visited>,
    >,
    mut edge_query: Query<
        (
            &RoadEdge,
            Option<&mut Sprite>,
            Option<&MeshMaterial2d<ColorMaterial>>,
        ),
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let car_transform = car_query.single().unwrap();
    let car_corners = get_car_corners(car_transform);

    for (entity, road_transform, road_segment) in road_query.iter_mut() {
        // Check if ANY corner of the car is on this segment
        let mut car_touches_segment = false;

        for corner in car_corners.iter() {
            let local_corner = world_to_local_2d(road_transform, *corner);

            if is_point_in_segment(local_corner, road_segment.segment_type) {
                car_touches_segment = true;
                break;
            }
        }

        if car_touches_segment {
            // Insert Visited marker component
            commands.entity(entity).insert(Visited);

            // Update colors of all edges belonging to this segment
            for (road_edge, sprite_opt, material_opt) in edge_query.iter_mut() {
                if road_edge.parent_segment == entity {
                    // Update color based on edge type:
                    // - Straight segment edges use Sprite component
                    // - Corner segment edges use MeshMaterial2d<ColorMaterial>
                    if let Some(mut sprite) = sprite_opt {
                        sprite.color = VISITED_EDGE_COLOR;
                    } else if let Some(material_handle) = material_opt {
                        if let Some(material) = materials.get_mut(&material_handle.0) {
                            material.color = VISITED_EDGE_COLOR;
                        }
                    }
                }
            }
        }
    }
}
