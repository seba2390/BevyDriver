use bevy::prelude::*;
use crate::car::constants::CAR_HEIGHT;
use crate::car::systems::spawn_car;
use crate::constants::{CurrentLevel, GameState, ResumeFromPause};
use crate::hud::systems::{
    check_finish_line_crossing, handle_off_road_logic, init_race_state, spawn_controls_hint,
    spawn_level_text_ui, spawn_multiplier_ui, spawn_nos_boost_bar, spawn_nos_boost_bar_glow,
    spawn_timer_ui,
};
use crate::road::components::{Direction, Track};
use crate::road::systems::{
    check_car_on_road, spawn_finish_line, spawn_start_line, spawn_track,
    update_segment_visited_status,
};
use crate::props::systems::{rotate_powerups, check_powerup_collision};
use crate::road::track_generator::{generate_random_track, TrackGeneratorConfig};
use crate::road::tracks::get_track;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (setup_game.run_if(not(resuming_from_pause)), clear_resume_flag).chain(),
        )
        .add_systems(
            Update,
            (
                check_car_on_road.pipe(handle_off_road_logic),
                (update_segment_visited_status, check_finish_line_crossing).chain(),
                rotate_powerups,
                check_powerup_collision,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
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
            prop_indices: generated.prop_indices,
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

    spawn_timer_ui(&mut commands);
    spawn_multiplier_ui(&mut commands);
    spawn_nos_boost_bar(&mut commands);
    spawn_nos_boost_bar_glow(&mut commands);
    spawn_controls_hint(&mut commands);
    spawn_level_text_ui(&mut commands, &current_level);
    init_race_state(&mut commands, track.starting_point.y);
}
