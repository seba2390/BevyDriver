use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::prelude::*;

mod car;
mod constants;
mod hud;
mod level_complete;
mod load_menu;
mod name_entry;
mod pause_menu;
mod save;
mod start_menu;
mod road;
mod styles;
mod utils;

use car::constants::CAR_HEIGHT;
use car::systems::{handle_input, move_car, spawn_car};
use constants::{CurrentLevel, GameState, ResumeFromPause, WINDOW_HEIGHT, WINDOW_WIDTH, BLOOM_INTENSITY};
use hud::systems::{
    check_finish_line_crossing, check_race_finished, check_start_line_crossing,
    handle_off_road_logic, init_race_state, spawn_multiplier_ui, spawn_off_road_ui, spawn_timer_ui,
    tick_race_timer, update_multiplier_display, update_timer_display,
};
use level_complete::systems::{
    level_complete_action,
    spawn_level_complete_menu,
};
use level_complete::components::OnLevelCompleteScreen;
use pause_menu::components::OnPauseMenuScreen;
use pause_menu::systems::{
    handle_pause_input, handle_resume_input, pause_menu_action, spawn_pause_menu,
};
use load_menu::components::OnLoadMenuScreen;
use load_menu::systems::{
    cleanup_load_menu, handle_delete_click, handle_save_slot_click, load_menu_action,
    spawn_load_menu, handle_delete_confirm_action,
};
use name_entry::components::OnNameEntryScreen;
use name_entry::systems::{
    cleanup_name_entry, handle_name_input, name_entry_action,
    spawn_name_entry,
};
use save::CurrentSave;
use start_menu::systems::{menu_action, spawn_menu};
use styles::menu::standard_button_system;
use start_menu::components::{OnMenuScreen, GameEntity};
use utils::despawn_all;
use road::components::{Direction, Track};
use road::systems::{check_car_on_road, spawn_finish_line, spawn_start_line, spawn_track, update_segment_visited_status};
use road::tracks::get_track;
use road::track_generator::{generate_random_track, TrackGeneratorConfig};

use crate::hud::systems::spawn_level_text_ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Driver".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        // Initialize game state (starts in Menu by default)
        .init_state::<GameState>()
        // Initialize current level resource
        .init_resource::<CurrentLevel>()
        // Initialize current save resource
        .init_resource::<CurrentSave>()
        // Initialize resume from pause flag
        .init_resource::<ResumeFromPause>()
        // Spawn camera once on startup (persists across states)
        .add_systems(Startup, spawn_camera)
        // Menu state systems
        .add_systems(OnEnter(GameState::StartMenu), spawn_menu)
        .add_systems(OnExit(GameState::StartMenu), despawn_all::<OnMenuScreen>)
        .add_systems(
            Update,
            (standard_button_system, menu_action).run_if(in_state(GameState::StartMenu)),
        )
        // Name Entry state systems
        .add_systems(OnEnter(GameState::NewGameNameEntry), spawn_name_entry)
        .add_systems(OnExit(GameState::NewGameNameEntry), (despawn_all::<OnNameEntryScreen>, cleanup_name_entry))
        .add_systems(
            Update,
            (handle_name_input, standard_button_system, name_entry_action)
                .run_if(in_state(GameState::NewGameNameEntry)),
        )
        // Load Menu state systems
        .add_systems(OnEnter(GameState::LoadGameMenu), spawn_load_menu)
        .add_systems(OnExit(GameState::LoadGameMenu), (despawn_all::<OnLoadMenuScreen>, cleanup_load_menu))
        .add_systems(
            Update,
            (
                standard_button_system,
                handle_save_slot_click,
                handle_delete_click,
                handle_delete_confirm_action,
                load_menu_action,
            )
                .run_if(in_state(GameState::LoadGameMenu)),
        )
        // Playing state systems
        .add_systems(OnEnter(GameState::Playing), (
            setup_game.run_if(not(resuming_from_pause)),
            clear_resume_flag,
        ).chain())
        // Note: Game cleanup happens in cleanup_level_complete_menu to keep track visible during overlay
        .add_systems(
            Update,
            (
                handle_input,
                move_car,
                check_car_on_road.pipe(handle_off_road_logic),
                // Chain these two systems to ensure Visited markers are applied
                // before checking if all segments are visited at the finish line
                (update_segment_visited_status, check_finish_line_crossing).chain(),
                check_start_line_crossing,
                tick_race_timer,
                update_timer_display,
                update_multiplier_display,
                check_race_finished,
                handle_pause_input,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Paused state systems
        .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
        .add_systems(OnExit(GameState::Paused), despawn_all::<OnPauseMenuScreen>)
        .add_systems(
            Update,
            (standard_button_system, pause_menu_action, handle_resume_input)
                .run_if(in_state(GameState::Paused)),
        )
        // Level Complete state systems
        .add_systems(OnEnter(GameState::LevelComplete), spawn_level_complete_menu)
        .add_systems(OnExit(GameState::LevelComplete), (despawn_all::<OnLevelCompleteScreen>, despawn_all::<GameEntity>))
        .add_systems(
            Update,
            (standard_button_system, level_complete_action)
                .run_if(in_state(GameState::LevelComplete)),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: BLOOM_INTENSITY,
            // Use prefilter to only bloom pixels above threshold (more localized)
            prefilter: BloomPrefilter {
                threshold: 1.1,           // Only bloom pixels brighter than 1.0 (HDR)
                threshold_softness: 0.2,  // Soft transition
            },
            // Additive mode works better with prefilter for localized glow
            composite_mode: BloomCompositeMode::Additive,
            // Tighten the scatter for more localized glow
            high_pass_frequency: 0.45,
            ..default()
        },
        DebandDither::Enabled,
    ));
}

/// Run condition: returns true if we're resuming from pause
fn resuming_from_pause(resume_flag: Res<ResumeFromPause>) -> bool {
    resume_flag.0
}

/// Clears the resume flag after entering Playing state
fn clear_resume_flag(mut resume_flag: ResMut<ResumeFromPause>) {
    resume_flag.0 = false;
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_level: Res<CurrentLevel>,
) {
    // Use hardcoded tracks for levels 1-3, random tracks for level 4+
    let track: Track = if current_level.0 <= 3 {
        get_track(current_level.0)
    } else {
        // Generate random track with seed based on level number
        let config = TrackGeneratorConfig {
            min_segments: 50,
            max_segments: 120,
            target_difficulty: 0.5,
            seed: current_level.0 as u64,
        };
        let generated = generate_random_track(&config)
            .expect("Failed to generate random track");

        Track {
            layout: generated.layout,
            starting_point: generated.starting_point,
        }
    };

    spawn_car(&mut commands, track.starting_point);
    spawn_track(&mut commands, &mut meshes, &mut materials, &track);

    // Spawn start line at the track's starting point (car crosses going up)
    spawn_start_line(&mut commands, track.starting_point, Direction::Up);

    // Place finish line just behind the car's initial position
    // Car needs to complete the lap and cross this line from below
    let finish_position = Vec2::new(
        track.starting_point.x,
        track.starting_point.y - CAR_HEIGHT, // Just behind the car
    );
    spawn_finish_line(&mut commands, finish_position, Direction::Up);

    spawn_off_road_ui(&mut commands);
    spawn_timer_ui(&mut commands);
    spawn_multiplier_ui(&mut commands);
    spawn_level_text_ui(&mut commands, &current_level);
    init_race_state(commands, track.starting_point.y);
}
