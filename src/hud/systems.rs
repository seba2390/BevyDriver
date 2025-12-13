use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::car::components::Car;
use crate::constants::{CurrentLevel, GameState};
use crate::hud::components::{MultiplierText, OffRoadText, LevelText, RaceState, RaceStatus, TimerText};
use crate::hud::helpers::{format_elapsed_time, has_crossed_line, is_within_line_x_bounds};
use crate::start_menu::components::GameEntity;
use crate::road::components::{Direction, FinishLine, RoadSegment, StartLine, Visited};
use crate::styles::hud::{multiplier_style, off_road_warning_style, level_text_style, timer_color, timer_style};

/// Spawns the off the road level text UI element
pub fn spawn_level_text_ui(commands: &mut Commands, current_level: &CurrentLevel) {
    commands.spawn((
        Text::new(format!("Level {}", current_level.0)),
        level_text_style(),
        LevelText,
        GameEntity,
    ));
}

/// Spawns the off-road warning UI element
pub fn spawn_off_road_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("Off the road!"),
        off_road_warning_style(),
        Visibility::Hidden,
        OffRoadText,
        GameEntity,
    ));
}

/// Spawns the timer UI element in the upper right corner
pub fn spawn_timer_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("0.00"),
        timer_style(),
        TimerText,
        GameEntity,
    ));
}

/// Spawns the multiplier indicator UI element below the timer
pub fn spawn_multiplier_ui(commands: &mut Commands) {
    use crate::hud::constants::OFF_ROAD_TIME_MULTIPLIER;

    commands.spawn((
        Text::new(format!("(x{})", OFF_ROAD_TIME_MULTIPLIER as i32)),
        multiplier_style(),
        Visibility::Hidden,
        MultiplierText,
        GameEntity,
    ));
}

/// Initialize the race state resource
pub fn init_race_state(mut commands: Commands, car_start_y: f32) {
    commands.insert_resource(RaceState {
        stopwatch: Stopwatch::new(),
        status: RaceStatus::WaitingToStart,
        final_time: None,
        car_last_y: car_start_y,
        is_on_road: true,
    });
}

/// Checks if the car crossed a line (used for start/finish detection)
fn has_crossed_line_at(
    car_pos: Vec2,
    car_last_y: f32,
    line_pos: Vec2,
    direction: Direction,
) -> bool {
    let within_x_bounds = is_within_line_x_bounds(car_pos.x, line_pos.x);
    let crossed = has_crossed_line(car_pos.y, car_last_y, line_pos.y, direction);
    return within_x_bounds && crossed;
}

/// System to check if the car crosses the start line and start the timer
pub fn check_start_line_crossing(
    car_query: Single<&Transform, With<Car>>,
    start_line_query: Single<(&Transform, &StartLine)>,
    mut race_state: ResMut<RaceState>,
) {
    // Only check for start crossing before race begins
    if race_state.status != RaceStatus::WaitingToStart {
        return;
    }

    let car_pos = car_query.translation.truncate();
    let (start_transform, start_line) = *start_line_query;
    let start_pos = start_transform.translation.truncate();

    // Check if car crossed the start line
    if has_crossed_line_at(car_pos, race_state.car_last_y, start_pos, start_line.direction) {
        race_state.start_race();
    }

    // Update last Y position for next frame's crossing detection
    race_state.set_previous_car_y(car_pos.y);
}

/// System to check if the car crosses the finish line and stop the timer
pub fn check_finish_line_crossing(
    car_query: Single<&Transform, With<Car>>,
    finish_line_query: Single<(&Transform, &FinishLine)>,
    unvisited_query: Query<(), (With<RoadSegment>, Without<Visited>)>,
    mut race_state: ResMut<RaceState>,
) {
    // Only check for finish crossing while actively racing
    if race_state.status != RaceStatus::Racing {
        return;
    }

    let car_pos = car_query.translation.truncate();
    let (finish_transform, finish_line) = *finish_line_query;
    let finish_pos = finish_transform.translation.truncate();

    // Check if car crossed the finish line AND all segments have been visited
    if has_crossed_line_at(car_pos, race_state.car_last_y, finish_pos, finish_line.direction) {
        // All segments visited = no unvisited segments remain (O(1) check)
        let all_visited = unvisited_query.is_empty();
        if all_visited {
            race_state.finish_race();
        }
    }

    // Update last Y position for next frame's crossing detection
    race_state.set_previous_car_y(car_pos.y);
}

/// System to tick the race timer
pub fn tick_race_timer(mut race_state: ResMut<RaceState>, time: Res<Time>) {
    use crate::hud::constants::OFF_ROAD_TIME_MULTIPLIER;

    if race_state.status == RaceStatus::Racing {
        let multiplier = if race_state.is_on_road { 1.0 } else { OFF_ROAD_TIME_MULTIPLIER };
        race_state.stopwatch.tick(time.delta().mul_f32(multiplier));
    }
}

/// System to update the timer display
pub fn update_timer_display(
    race_state: Res<RaceState>,
    mut query: Query<(&mut Text, &mut TextColor), With<TimerText>>,
) {
    if let Ok((mut text, mut color)) = query.single_mut() {
        let elapsed = match race_state.status {
            RaceStatus::Finished => race_state.final_time.unwrap_or(0.0),
            _ => race_state.stopwatch.elapsed_secs(),
        };
        **text = format_elapsed_time(elapsed);
        *color = timer_color(&race_state.status);
    }
}

/// Handles showing/hiding the off-road warning based on car position
pub fn handle_off_road_logic(
    In(is_on_road): In<bool>,
    mut race_state: ResMut<RaceState>,
    mut query: Query<&mut Visibility, With<OffRoadText>>,
) {
    // Update race state with current road status
    race_state.is_on_road = is_on_road;

    if let Ok(mut visibility) = query.single_mut() {
        if is_on_road {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

/// System to update the multiplier display visibility based on road status
pub fn update_multiplier_display(
    race_state: Res<RaceState>,
    mut query: Query<&mut Visibility, With<MultiplierText>>,
) {
    if let Ok(mut visibility) = query.single_mut() {
        // Only show multiplier when racing and off the road
        if race_state.status == RaceStatus::Racing && !race_state.is_on_road {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// System to detect when race finishes and transition to LevelComplete state
pub fn check_race_finished(
    race_state: Res<RaceState>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if race_state.status == RaceStatus::Finished {
        game_state.set(GameState::LevelComplete);
    }
}
