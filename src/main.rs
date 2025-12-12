use bevy::prelude::*;

mod car;
mod constants;
mod hud;
mod road;

use car::constants::CAR_HEIGHT;
use car::systems::{handle_input, move_car, spawn_car};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use hud::systems::{
    check_finish_line_crossing, check_start_line_crossing, handle_off_road_logic, init_race_state,
    spawn_off_road_ui, spawn_timer_ui, tick_race_timer, update_timer_display,
};
use road::components::Direction;
use road::systems::{check_car_on_road, spawn_finish_line, spawn_start_line, spawn_track};
use road::tracks::{TRACK_2, TRACK_3};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Driving Game".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
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
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let track = &TRACK_3;

    commands.spawn(Camera2d);
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
