use bevy::prelude::*;
use bevy::time::Stopwatch;

/// Marker component for the "Off the road!" warning text
#[derive(Component)]
pub struct LevelText;

/// Marker component for the "Off the road!" warning text
#[derive(Component)]
pub struct OffRoadText;

/// Marker component for the lap timer display text
#[derive(Component)]
pub struct TimerText;

/// Marker component for the time multiplier indicator text
#[derive(Component)]
pub struct MultiplierText;

/// The current status of the race
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum RaceStatus {
    /// Race hasn't started yet, waiting for car to cross start line
    #[default]
    WaitingToStart,
    /// Race is in progress, timer is running
    Racing,
    /// Race is finished, timer has stopped
    Finished,
}

/// Resource that tracks the race state and timing
#[derive(Resource)]
pub struct RaceState {
    /// The stopwatch tracking elapsed time
    pub stopwatch: Stopwatch,
    /// Current status of the race
    pub status: RaceStatus,
    /// The final recorded time (set when race finishes)
    pub final_time: Option<f32>,
    /// Car's Y position last frame (for crossing detection)
    pub car_last_y: f32,
    /// Whether the car is currently on the road (affects timer speed)
    pub is_on_road: bool,
}

impl RaceState {

    pub fn start_race(&mut self) {
        self.status = RaceStatus::Racing;
        self.stopwatch.reset();
        self.stopwatch.unpause();
    }

    pub fn finish_race(&mut self) {
        self.status = RaceStatus::Finished;
        self.stopwatch.pause();
        self.final_time = Some(self.stopwatch.elapsed_secs());
    }

    /// Stores the car's Y position from this frame for next frame's crossing detection.
    /// Crossing = car moved from one side of a line to the other between frames.
    pub fn set_previous_car_y(&mut self, y: f32) {
        self.car_last_y = y;
    }

}
