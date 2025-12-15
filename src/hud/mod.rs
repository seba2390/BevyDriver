pub mod components;
pub mod constants;
pub mod helpers;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use systems::{
    check_race_finished, check_start_line_crossing, render_controls_hint_arrows, tick_race_timer,
    update_controls_hint, update_multiplier_display, update_timer_display,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_start_line_crossing,
                tick_race_timer,
                update_timer_display,
                update_multiplier_display,
                update_controls_hint,
                render_controls_hint_arrows,
                check_race_finished,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
