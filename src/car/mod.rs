pub mod components;
pub mod constants;
pub mod helpers;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use systems::{handle_input, move_car};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_input, move_car).run_if(in_state(GameState::Playing)),
        );
    }
}
