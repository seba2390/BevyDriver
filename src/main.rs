use bevy::prelude::*;

mod car;
mod constants;
mod hud;
mod level_complete;
mod start_menu;
mod road;
mod styles;

use car::constants::CAR_HEIGHT;
use car::systems::{handle_input, move_car, spawn_car};
use constants::{CurrentLevel, GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use hud::systems::{
    check_finish_line_crossing, check_race_finished, check_start_line_crossing,
    handle_off_road_logic, init_race_state, spawn_off_road_ui, spawn_timer_ui,
    tick_race_timer, update_timer_display,
};
use level_complete::systems::{
    cleanup_level_complete_menu, level_complete_action, level_complete_button_system,
    spawn_level_complete_menu,
};
use start_menu::systems::{button_system, cleanup_menu, menu_action, spawn_menu};
use road::components::Direction;
use road::systems::{check_car_on_road, spawn_finish_line, spawn_start_line, spawn_track};
use road::tracks::get_track;

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
        // Spawn camera once on startup (persists across states)
        .add_systems(Startup, spawn_camera)
        // Menu state systems
        .add_systems(OnEnter(GameState::StartMenu), spawn_menu)
        .add_systems(OnExit(GameState::StartMenu), cleanup_menu)
        .add_systems(
            Update,
            (button_system, menu_action).run_if(in_state(GameState::StartMenu)),
        )
        // Playing state systems
        .add_systems(OnEnter(GameState::Playing), setup_game)
        // Note: Game cleanup happens in cleanup_level_complete_menu to keep track visible during overlay
        .add_systems(
            Update,
            (
                handle_input,
                move_car,
                check_car_on_road.pipe(handle_off_road_logic),
                check_start_line_crossing,
                check_finish_line_crossing,
                tick_race_timer,
                update_timer_display,
                check_race_finished,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Level Complete state systems
        .add_systems(OnEnter(GameState::LevelComplete), spawn_level_complete_menu)
        .add_systems(OnExit(GameState::LevelComplete), cleanup_level_complete_menu)
        .add_systems(
            Update,
            (level_complete_button_system, level_complete_action)
                .run_if(in_state(GameState::LevelComplete)),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_level: Res<CurrentLevel>,
) {
    let track = get_track(current_level.0);

    spawn_car(&mut commands, track.starting_point);
    spawn_track(&mut commands, &mut meshes, &mut materials, track);

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
    spawn_level_text_ui(&mut commands, &current_level);
    init_race_state(commands, track.starting_point.y);
}
