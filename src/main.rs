use bevy::prelude::*;

mod car;
mod constants;
mod hud;
mod menu;
mod road;
mod styles;

use car::constants::CAR_HEIGHT;
use car::systems::{handle_input, move_car, spawn_car};
use constants::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use hud::systems::{
    check_finish_line_crossing, check_start_line_crossing, handle_off_road_logic, init_race_state,
    spawn_off_road_ui, spawn_timer_ui, tick_race_timer, update_timer_display,
};
use menu::systems::{button_system, cleanup_game, cleanup_menu, menu_action, spawn_menu};
use road::components::Direction;
use road::systems::{check_car_on_road, spawn_finish_line, spawn_start_line, spawn_track};
use road::tracks::TRACK_1;

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
        // Spawn camera once on startup (persists across states)
        .add_systems(Startup, spawn_camera)
        // Menu state systems
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(
            Update,
            (button_system, menu_action).run_if(in_state(GameState::Menu)),
        )
        // Playing state systems
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(OnExit(GameState::Playing), cleanup_game)
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
            )
                .run_if(in_state(GameState::Playing)),
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
) {
    let track = &TRACK_1;

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
    init_race_state(commands, track.starting_point.y);
}
