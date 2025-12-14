use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::car::components::Car;
use crate::car::components::Velocity;
use crate::constants::{CurrentLevel, GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::hud::components::{
    ControlsHint, LevelText, MultiplierText, OffRoadText, RaceState, RaceStatus, TimerText,
};
use crate::hud::constants::{
    ARROW_BASE_X_OFFSET, ARROW_BASE_Y_OFFSET, ARROW_HEAD_SIZE, ARROW_SIZE, ARROW_STEER_OFFSET,
    ARROW_VERTICAL_OFFSET, CONTROLS_FADE_DELAY, CONTROLS_FADE_DURATION, CONTROLS_HINT_ALPHA,
    CONTROLS_HINT_LINE_HEIGHT, CONTROLS_HINT_PADDING, CONTROLS_HINT_RGB, CONTROL_LABELS,
    OFF_ROAD_TIME_MULTIPLIER, PLAYER_MOVED_VELOCITY_THRESHOLD,
};
use crate::hud::helpers::{format_elapsed_time, has_crossed_line, is_within_line_x_bounds};
use crate::road::components::{Direction, FinishLine, RoadSegment, StartLine, Visited};
use crate::start_menu::components::GameEntity;
use crate::styles::hud::{
    controls_hint_line_style, level_text_style, multiplier_style, off_road_warning_style,
    timer_color, timer_style,
};

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
    if race_state.status == RaceStatus::Racing {
        let multiplier = if race_state.is_on_road {
            1.0
        } else {
            OFF_ROAD_TIME_MULTIPLIER
        };
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

// ============================================================================
// Controls Hint
// ============================================================================

/// Spawns the controls hint UI elements - one text per line in the bottom-left corner
pub fn spawn_controls_hint(commands: &mut Commands) {
    for (i, label) in CONTROL_LABELS.iter().enumerate() {
        commands.spawn((
            Text::new(*label),
            controls_hint_line_style(i),
            ControlsHint {
                timer: 0.0,
                fade_delay: CONTROLS_FADE_DELAY,
                fade_duration: CONTROLS_FADE_DURATION,
            },
            GameEntity,
        ));
    }
}

/// Draws a 2D arrow using gizmos
fn draw_arrow_2d(gizmos: &mut Gizmos, start: Vec2, direction: Vec2, size: f32, color: Color) {
    let end = start + direction.normalize() * size;

    // Main line
    gizmos.line_2d(start, end, color);

    // Arrow head (two lines forming a V)
    let perp = Vec2::new(-direction.y, direction.x).normalize();
    let head_back = end - direction.normalize() * ARROW_HEAD_SIZE;
    let head_left = head_back + perp * (ARROW_HEAD_SIZE * 0.5);
    let head_right = head_back - perp * (ARROW_HEAD_SIZE * 0.5);

    gizmos.line_2d(end, head_left, color);
    gizmos.line_2d(end, head_right, color);
}

/// Renders the arrow gizmos for controls hint
pub fn render_controls_hint_arrows(mut gizmos: Gizmos, hint_query: Query<&ControlsHint>) {
    // Check if any hints exist
    let Ok(hint) = hint_query.iter().next().ok_or(()) else {
        return;
    };

    // Calculate alpha based on fade progress
    let alpha = if hint.timer >= hint.fade_delay {
        let fade_progress = (hint.timer - hint.fade_delay) / hint.fade_duration;
        if fade_progress >= 1.0 {
            return; // Fully faded, don't draw
        }
        CONTROLS_HINT_ALPHA * (1.0 - fade_progress)
    } else {
        CONTROLS_HINT_ALPHA
    };

    let color = Color::srgba(CONTROLS_HINT_RGB.0, CONTROLS_HINT_RGB.1, CONTROLS_HINT_RGB.2, alpha);

    // Position in bottom-left corner (screen space -> world space for 2D)
    // The camera is at origin, so we need to offset from center
    let base_x = -(WINDOW_WIDTH as f32) / 2.0 + CONTROLS_HINT_PADDING + ARROW_BASE_X_OFFSET;
    let base_y = -(WINDOW_HEIGHT as f32) / 2.0 + CONTROLS_HINT_PADDING + ARROW_BASE_Y_OFFSET;

    // Line 0: Up arrow (Accelerate) - top line
    let line0_y = base_y + CONTROLS_HINT_LINE_HEIGHT * 3.0;
    draw_arrow_2d(
        &mut gizmos,
        Vec2::new(base_x, line0_y - ARROW_VERTICAL_OFFSET),
        Vec2::Y,
        ARROW_SIZE,
        color,
    );

    // Line 1: Left and Right arrows (Steer)
    let line1_y = base_y + CONTROLS_HINT_LINE_HEIGHT * 2.0;
    draw_arrow_2d(
        &mut gizmos,
        Vec2::new(base_x - ARROW_STEER_OFFSET, line1_y),
        -Vec2::X,
        ARROW_SIZE,
        color,
    );
    draw_arrow_2d(
        &mut gizmos,
        Vec2::new(base_x + ARROW_STEER_OFFSET, line1_y),
        Vec2::X,
        ARROW_SIZE,
        color,
    );

    // Line 2: Down arrow (Brake)
    let line2_y = base_y + CONTROLS_HINT_LINE_HEIGHT * 1.0;
    draw_arrow_2d(
        &mut gizmos,
        Vec2::new(base_x, line2_y + ARROW_VERTICAL_OFFSET),
        -Vec2::Y,
        ARROW_SIZE,
        color,
    );

    // Line 3: ESC text (no arrow needed, relies on text label)
}

/// Updates the controls hint - fades out after delay or when player moves
pub fn update_controls_hint(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    car_query: Query<&Velocity, With<Car>>,
    mut hint_query: Query<(Entity, &mut ControlsHint, &mut TextColor)>,
) {
    if hint_query.is_empty() {
        return;
    }

    // Check if player has started moving (any arrow key or car has velocity)
    let player_moved = keyboard.any_pressed([
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ]) || car_query
        .iter()
        .any(|v| v.0.length_squared() > PLAYER_MOVED_VELOCITY_THRESHOLD);

    let delta = time.delta_secs();

    for (entity, mut hint, mut color) in hint_query.iter_mut() {
        // If player moved, start fading immediately
        if player_moved && hint.timer < hint.fade_delay {
            hint.timer = hint.fade_delay;
        }

        hint.timer += delta;

        // Calculate alpha based on fade progress
        if hint.timer >= hint.fade_delay {
            let fade_progress = (hint.timer - hint.fade_delay) / hint.fade_duration;
            if fade_progress >= 1.0 {
                // Fully faded - despawn the hint
                commands.entity(entity).despawn();
            } else {
                // Update alpha
                let alpha = CONTROLS_HINT_ALPHA * (1.0 - fade_progress);
                color.0 =
                    Color::srgba(CONTROLS_HINT_RGB.0, CONTROLS_HINT_RGB.1, CONTROLS_HINT_RGB.2, alpha);
            }
        }
    }
}
