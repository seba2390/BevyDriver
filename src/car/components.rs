use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Component)]
pub struct Car;

#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Component indicating the car has collected a NOS powerup and can activate boost.
/// The timer counts down the availability window - pressing SPACE activates the boost.
#[derive(Component)]
pub struct NosBoostAvailable {
    /// Timer counting down the availability window
    pub timer: Timer,
    /// Whether the boost is currently active (SPACE held)
    pub active: bool,
}

impl NosBoostAvailable {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, bevy::time::TimerMode::Once),
            active: false,
        }
    }

    /// Returns the remaining fraction (1.0 = full, 0.0 = empty)
    pub fn remaining_fraction(&self) -> f32 {
        1.0 - self.timer.fraction()
    }
}
