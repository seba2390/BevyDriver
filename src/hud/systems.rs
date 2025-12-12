use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::car::components::Car;
use crate::hud::components::{OffRoadText, RaceState, RaceStatus, TimerText};
use crate::hud::helpers::{format_elapsed_time, has_crossed_line, is_within_line_x_bounds};
use crate::menu::components::GameEntity;
use crate::road::components::{Direction, FinishLine, StartLine};

/// Spawns the off-road warning UI element
pub fn spawn_off_road_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("Off the road!"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.2, 0.2)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        Visibility::Hidden,
        OffRoadText,
        GameEntity,
    ));
}

/// Spawns the timer UI element in the upper right corner
pub fn spawn_timer_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("0.00"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        TextLayout::new_with_justify(Justify::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        TimerText,
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
    });
}

/// Helper to check if the car crossed any line in the given iterator.
/// Returns Some((line_x, line_y, direction)) if a crossing was detected.
fn detect_line_crossing<'a>(
    car_pos: Vec2,
    car_last_y: f32,
    lines: impl Iterator<Item = (Vec2, Direction)>,
) -> Option<Vec2> {
    for (line_pos, direction) in lines {
        let within_x_bounds = is_within_line_x_bounds(car_pos.x, line_pos.x);
        let crossed = has_crossed_line(car_pos.y, car_last_y, line_pos.y, direction);

        if within_x_bounds && crossed {
            return Some(line_pos);
        }
    }
    None
}

/// System to check if the car crosses the start line and start the timer
pub fn check_start_line_crossing(
    car_query: Query<&Transform, With<Car>>,
    start_line_query: Query<(&Transform, &StartLine)>,
    mut race_state: ResMut<RaceState>,
) {
    if race_state.status != RaceStatus::WaitingToStart {
        return;
    }

    let Ok(car_transform) = car_query.single() else {
        return;
    };

    let car_pos = car_transform.translation.truncate();
    let lines = start_line_query
        .iter()
        .map(|(t, sl)| (t.translation.truncate(), sl.direction));

    if detect_line_crossing(car_pos, race_state.car_last_y, lines).is_some() {
        race_state.status = RaceStatus::Racing;
        race_state.stopwatch.reset();
        race_state.stopwatch.unpause();
    }

    race_state.car_last_y = car_pos.y;
}

/// System to check if the car crosses the finish line and stop the timer
pub fn check_finish_line_crossing(
    car_query: Query<&Transform, With<Car>>,
    finish_line_query: Query<(&Transform, &FinishLine)>,
    mut race_state: ResMut<RaceState>,
) {
    if race_state.status != RaceStatus::Racing {
        return;
    }

    let Ok(car_transform) = car_query.single() else {
        return;
    };

    let car_pos = car_transform.translation.truncate();
    let lines = finish_line_query
        .iter()
        .map(|(t, fl)| (t.translation.truncate(), fl.direction));

    if detect_line_crossing(car_pos, race_state.car_last_y, lines).is_some() {
        race_state.status = RaceStatus::Finished;
        race_state.stopwatch.pause();
        race_state.final_time = Some(race_state.stopwatch.elapsed_secs());
    }

    race_state.car_last_y = car_pos.y;
}

/// System to tick the race timer
pub fn tick_race_timer(mut race_state: ResMut<RaceState>, time: Res<Time>) {
    if race_state.status == RaceStatus::Racing {
        race_state.stopwatch.tick(time.delta());
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

        // Change color based on race status
        *color = match race_state.status {
            RaceStatus::WaitingToStart => TextColor(Color::srgb(0.7, 0.7, 0.7)), // Gray
            RaceStatus::Racing => TextColor(Color::srgb(1.0, 1.0, 1.0)),         // White
            RaceStatus::Finished => TextColor(Color::srgb(0.2, 1.0, 0.2)),       // Green
        };
    }
}

/// Handles showing/hiding the off-road warning based on car position
pub fn handle_off_road_logic(
    In(is_on_road): In<bool>,
    mut query: Query<&mut Visibility, With<OffRoadText>>,
) {
    if let Ok(mut visibility) = query.single_mut() {
        if is_on_road {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}
