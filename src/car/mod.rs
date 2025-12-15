pub mod components;
pub mod constants;
pub mod helpers;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use systems::{handle_input, move_car, update_nos_boost};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_input, update_nos_boost, move_car)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}
