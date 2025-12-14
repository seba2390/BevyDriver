//! Minimap rendering for the level menu.
//! Renders scaled-down track previews with full bloom/glow effects.

use bevy::camera::{visibility::RenderLayers, RenderTarget};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::image::{ImageSampler, ImageSamplerDescriptor};
use bevy::math::primitives::CircularSector;
use bevy::post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use std::collections::HashMap;

use crate::constants::{BLOOM_INTENSITY, GAME_BACKGROUND_COLOR};
use crate::level_menu::constants::{MINI_MAP_HEIGHT, MINI_MAP_WIDTH};
use crate::road::components::{Direction, RoadSegmentType, Track};
use crate::road::constants::{
    ROAD_EDGE_WIDTH, ROAD_SEGMENT_COLOR, ROAD_SEGMENT_LENGTH, ROAD_WIDTH, VISITED_EDGE_COLOR,
};
use crate::road::helpers::{compute_track_bounds, get_exit_direction, get_position_offset, get_rotation};
use crate::road::track_generator::{generate_random_track, TrackGeneratorConfig};
use crate::road::tracks::get_track;

// ============================================================================
// Resources
// ============================================================================

/// Cache of rendered minimap images, keyed by level number.
#[derive(Resource, Default)]
pub struct MinimapCache {
    pub images: HashMap<usize, Handle<Image>>,
}

// ============================================================================
// Components
// ============================================================================

/// Marker for the minimap render camera.
#[derive(Component)]
pub struct MinimapCamera {
    pub level: usize,
}

/// Marker for entities that are part of a minimap scene (to be despawned after rendering).
#[derive(Component)]
pub struct MinimapSceneEntity {
    pub level: usize,
}

/// Marker for tracking when a minimap has been rendered.
#[derive(Component)]
pub struct MinimapRendered {
    pub level: usize,
    /// Number of frames to wait before capturing (for bloom to stabilize).
    pub frames_remaining: u32,
}

// ============================================================================
// Constants
// ============================================================================

/// Resolution multiplier for minimap rendering (higher = sharper but more memory).
const MINIMAP_RESOLUTION_SCALE: f32 = 2.0;

/// Number of frames to wait for bloom to stabilize before capturing.
const FRAMES_BEFORE_CAPTURE: u32 = 3;

/// Padding around the track in the minimap (as fraction of track size).
const MINIMAP_PADDING: f32 = 0.25;

/// Base render layer for minimap rendering (layers 1-31 are available, 0 is default).
/// We use layers 1+ for minimaps to isolate each level's track.
const MINIMAP_RENDER_LAYER_BASE: u8 = 1;

/// Get the render layer for a specific level's minimap.
/// Each level gets its own render layer to isolate its track from others.
fn get_minimap_render_layer(level: usize) -> RenderLayers {
    // Use layers 1-31 (0 is the default layer used by the main game)
    // Wrap around if we have more than 31 levels (unlikely but safe)
    let layer = MINIMAP_RENDER_LAYER_BASE as usize + ((level - 1) % 31);
    RenderLayers::layer(layer)
}

// ============================================================================
// Minimap Rendering
// ============================================================================

/// Creates a render target image for minimap rendering.
pub fn create_minimap_image(images: &mut Assets<Image>) -> Handle<Image> {
    let width = (MINI_MAP_WIDTH * MINIMAP_RESOLUTION_SCALE) as u32;
    let height = (MINI_MAP_HEIGHT * MINIMAP_RESOLUTION_SCALE) as u32;

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("minimap_render_target"),
            size,
            dimension: TextureDimension::D2,
            // Use Rgba8UnormSrgb to match swapchain format
            // This ensures tonemapping is applied correctly and stored as sRGB
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor::linear()),
        ..default()
    };

    // Fill with transparent background initially
    // Rgba8UnormSrgb uses 4 bytes per pixel
    image.resize(size);
    if let Some(ref mut data) = image.data {
        // Clear to zero (transparent black in float format)
        data.fill(0);
    }

    images.add(image)
}

/// Gets or generates the track for a given level.
pub fn get_level_track(level: usize) -> Track {
    if level <= 3 {
        get_track(level)
    } else {
        let config = TrackGeneratorConfig {
            min_segments: 50,
            max_segments: 120,
            target_difficulty: 0.5,
            seed: level as u64,
        };
        let generated = generate_random_track(&config).expect("Failed to generate random track");
        Track {
            layout: generated.layout,
            starting_point: generated.starting_point,
        }
    }
}

/// Calculates the scale and offset needed to fit a track into the minimap bounds.
pub fn calculate_minimap_transform(track: &Track) -> (f32, Vec2) {
    let (min, max) = compute_track_bounds(track.starting_point, &track.layout);

    // Add road width padding to bounds
    let padding = ROAD_WIDTH / 2.0 + ROAD_EDGE_WIDTH;
    let min = min - Vec2::splat(padding);
    let max = max + Vec2::splat(padding);

    let track_width = max.x - min.x;
    let track_height = max.y - min.y;
    let track_center = (min + max) / 2.0;

    // Calculate scale to fit in minimap with padding
    let available_width = MINI_MAP_WIDTH * MINIMAP_RESOLUTION_SCALE * (1.0 - MINIMAP_PADDING);
    let available_height = MINI_MAP_HEIGHT * MINIMAP_RESOLUTION_SCALE * (1.0 - MINIMAP_PADDING);

    let scale_x = available_width / track_width;
    let scale_y = available_height / track_height;
    let scale = scale_x.min(scale_y);

    (scale, track_center)
}

/// Spawns a minimap camera that renders to a texture.
pub fn spawn_minimap_camera(
    commands: &mut Commands,
    image_handle: Handle<Image>,
    level: usize,
    track_center: Vec2,
    scale: f32,
) -> Entity {
    // Camera needs to be positioned at track center and zoomed out
    // OrthographicProjection scale is inverse of our scale
    let projection_scale = 1.0 / scale;

    // Get the render layer for this level
    let render_layer = get_minimap_render_layer(level);

    let entity = commands
        .spawn((
            Camera2d,
            Tonemapping::TonyMcMapface,
            DebandDither::Enabled,
            MinimapCamera { level },
            render_layer,  // Only render entities on this layer
        ))
        .id();

    commands.entity(entity).insert((
        Camera {
            target: RenderTarget::from(image_handle.clone()),
            order: -1, // Render before main camera
            clear_color: ClearColorConfig::Custom(GAME_BACKGROUND_COLOR),
            ..default()
        },
        // Wrap OrthographicProjection in the Projection component
        Projection::Orthographic(OrthographicProjection {
            scale: projection_scale,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(track_center.x, track_center.y, 1000.0),
    ));

    commands.entity(entity).insert((
        Bloom {
            intensity: BLOOM_INTENSITY,
            prefilter: BloomPrefilter {
                threshold: 1.1,
                threshold_softness: 0.2,
            },
            composite_mode: BloomCompositeMode::Additive,
            high_pass_frequency: 0.45,
            ..default()
        },
        MinimapRendered {
            level,
            frames_remaining: FRAMES_BEFORE_CAPTURE,
        },
    ));

    entity
}

/// Spawns the minimap track scene (road segments with glowing edges).
pub fn spawn_minimap_track(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    track: &Track,
    level: usize,
) {
    let mut current_endpoint = Vec2::new(
        track.starting_point.x,
        track.starting_point.y - ROAD_SEGMENT_LENGTH / 2.0,
    );
    let mut current_direction = Direction::Up;

    for &segment_type in track.layout.iter() {
        match segment_type {
            RoadSegmentType::Straight => {
                current_endpoint = spawn_minimap_straight_road(
                    commands,
                    current_endpoint,
                    current_direction,
                    level,
                );
            }
            RoadSegmentType::CornerLeft | RoadSegmentType::CornerRight => {
                let (new_endpoint, new_direction) = spawn_minimap_corner_road(
                    commands,
                    meshes,
                    materials,
                    current_endpoint,
                    current_direction,
                    segment_type,
                    level,
                );
                current_endpoint = new_endpoint;
                current_direction = new_direction;
            }
        }
    }

    // Spawn start line marker
    spawn_minimap_start_line(commands, track.starting_point, level);
}

fn spawn_minimap_straight_road(
    commands: &mut Commands,
    current_endpoint: Vec2,
    current_direction: Direction,
    level: usize,
) -> Vec2 {
    let offset = get_position_offset(current_direction);
    let center = current_endpoint + offset / 2.0;

    let rotation = get_rotation(current_direction);
    let rotation_quat = Quat::from_rotation_z(rotation);
    let render_layer = get_minimap_render_layer(level);

    // Road surface
    commands.spawn((
        Sprite {
            color: ROAD_SEGMENT_COLOR,
            custom_size: Some(Vec2::new(ROAD_WIDTH, ROAD_SEGMENT_LENGTH)),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, 1.0).with_rotation(rotation_quat),
        MinimapSceneEntity { level },
        render_layer.clone(),
    ));

    // Glowing edges (always use VISITED_EDGE_COLOR for full glow)
    let perpendicular = rotation_quat.mul_vec3(Vec3::X).xy();
    let edge_offset = perpendicular * (ROAD_WIDTH / 2.0 + ROAD_EDGE_WIDTH / 2.0);

    // Left edge
    commands.spawn((
        Sprite {
            color: VISITED_EDGE_COLOR,
            custom_size: Some(Vec2::new(ROAD_EDGE_WIDTH, ROAD_SEGMENT_LENGTH)),
            ..default()
        },
        Transform::from_xyz(center.x - edge_offset.x, center.y - edge_offset.y, 1.2)
            .with_rotation(rotation_quat),
        MinimapSceneEntity { level },
        render_layer.clone(),
    ));

    // Right edge
    commands.spawn((
        Sprite {
            color: VISITED_EDGE_COLOR,
            custom_size: Some(Vec2::new(ROAD_EDGE_WIDTH, ROAD_SEGMENT_LENGTH)),
            ..default()
        },
        Transform::from_xyz(center.x + edge_offset.x, center.y + edge_offset.y, 1.2)
            .with_rotation(rotation_quat),
        MinimapSceneEntity { level },
        render_layer,
    ));

    current_endpoint + offset
}

fn spawn_minimap_corner_road(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    current_endpoint: Vec2,
    current_direction: Direction,
    segment_type: RoadSegmentType,
    level: usize,
) -> (Vec2, Direction) {
    let exit_direction = get_exit_direction(current_direction, segment_type);
    let exit_vec = get_direction_vector(exit_direction);
    let entry_vec = get_direction_vector(current_direction);

    let pivot = current_endpoint + exit_vec * (ROAD_WIDTH / 2.0);

    let sector = CircularSector::from_degrees(ROAD_WIDTH, 90.0);
    let render_layer = get_minimap_render_layer(level);

    let rotation_offset = match segment_type {
        RoadSegmentType::CornerRight => std::f32::consts::FRAC_PI_4,
        RoadSegmentType::CornerLeft => -std::f32::consts::FRAC_PI_4,
        _ => 0.0,
    };
    let rotation_angle = get_rotation(current_direction) + rotation_offset;

    // Road surface
    commands.spawn((
        Mesh2d(meshes.add(sector)),
        MeshMaterial2d(materials.add(ColorMaterial::from(ROAD_SEGMENT_COLOR))),
        Transform::from_xyz(pivot.x, pivot.y, 0.0)
            .with_rotation(Quat::from_rotation_z(rotation_angle)),
        MinimapSceneEntity { level },
        render_layer.clone(),
    ));

    // Outer glow arc
    let outer_radius = ROAD_WIDTH + ROAD_EDGE_WIDTH;
    let outer_sector = CircularSector::from_degrees(outer_radius, 90.0);
    let cutout_sector = CircularSector::from_degrees(ROAD_WIDTH, 90.0);

    commands.spawn((
        Mesh2d(meshes.add(outer_sector)),
        MeshMaterial2d(materials.add(ColorMaterial::from(VISITED_EDGE_COLOR))),
        Transform::from_xyz(pivot.x, pivot.y, 1.2)
            .with_rotation(Quat::from_rotation_z(rotation_angle)),
        MinimapSceneEntity { level },
        render_layer.clone(),
    ));

    // Cutout to create ring effect
    commands.spawn((
        Mesh2d(meshes.add(cutout_sector)),
        MeshMaterial2d(materials.add(ColorMaterial::from(ROAD_SEGMENT_COLOR))),
        Transform::from_xyz(pivot.x, pivot.y, 1.21)
            .with_rotation(Quat::from_rotation_z(rotation_angle)),
        MinimapSceneEntity { level },
        render_layer,
    ));

    let new_endpoint = pivot + entry_vec * (ROAD_WIDTH / 2.0);
    (new_endpoint, exit_direction)
}

fn get_direction_vector(direction: Direction) -> Vec2 {
    match direction {
        Direction::Up => Vec2::Y,
        Direction::Down => Vec2::NEG_Y,
        Direction::Left => Vec2::NEG_X,
        Direction::Right => Vec2::X,
    }
}

fn spawn_minimap_start_line(commands: &mut Commands, position: Vec2, level: usize) {
    let render_layer = get_minimap_render_layer(level);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2), // Green
            custom_size: Some(Vec2::new(ROAD_WIDTH, 2.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 1.5),
        MinimapSceneEntity { level },
        render_layer,
    ));
}

// ============================================================================
// Systems
// ============================================================================

/// System to initiate minimap rendering for levels that aren't cached.
pub fn setup_minimap_rendering(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    minimap_cache: Res<MinimapCache>,
    current_save: Res<crate::save::CurrentSave>,
    existing_cameras: Query<&MinimapCamera>,
) {
    let highest_level = current_save
        .0
        .as_ref()
        .map(|s| s.highest_level_unlocked)
        .unwrap_or(1);

    // Check which levels are already being rendered
    let rendering_levels: Vec<usize> = existing_cameras.iter().map(|c| c.level).collect();

    for level in 1..=highest_level {
        // Skip if already cached or being rendered
        if minimap_cache.images.contains_key(&level) || rendering_levels.contains(&level) {
            continue;
        }

        // Create render target
        let image_handle = create_minimap_image(&mut images);

        // Get track for this level
        let track = get_level_track(level);

        // Calculate transform
        let (scale, center) = calculate_minimap_transform(&track);

        // Spawn camera
        spawn_minimap_camera(&mut commands, image_handle, level, center, scale);

        // Spawn track scene
        spawn_minimap_track(&mut commands, &mut meshes, &mut materials, &track, level);
    }
}

/// System to capture rendered minimaps and clean up.
pub fn capture_minimaps(
    mut commands: Commands,
    mut minimap_cache: ResMut<MinimapCache>,
    mut cameras: Query<(Entity, &Camera, &mut MinimapRendered)>,
    scene_entities: Query<(Entity, &MinimapSceneEntity)>,
) {
    for (camera_entity, camera, mut rendered) in cameras.iter_mut() {
        if rendered.frames_remaining > 0 {
            rendered.frames_remaining -= 1;
            continue;
        }

        // Extract image handle from camera target
        if let RenderTarget::Image(image_render_target) = &camera.target {
            minimap_cache
                .images
                .insert(rendered.level, image_render_target.handle.clone());
        }

        // Despawn camera
        commands.entity(camera_entity).despawn();

        // Despawn scene entities for this level
        for (entity, scene_entity) in scene_entities.iter() {
            if scene_entity.level == rendered.level {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// System to clean up all minimap rendering resources when leaving the level menu.
pub fn cleanup_minimap_rendering(
    mut commands: Commands,
    cameras: Query<Entity, With<MinimapCamera>>,
    scene_entities: Query<Entity, With<MinimapSceneEntity>>,
) {
    for entity in cameras.iter() {
        commands.entity(entity).despawn();
    }
    for entity in scene_entities.iter() {
        commands.entity(entity).despawn();
    }
}
