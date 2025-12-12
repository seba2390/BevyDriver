use bevy::prelude::*;
use bevy::time::Stopwatch;

/// Marker component for the "Off the road!" warning text
#[derive(Component)]
pub struct OffRoadText;

/// Marker component for the lap timer display text
#[derive(Component)]
pub struct TimerText;

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
}
