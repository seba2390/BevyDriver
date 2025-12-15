pub mod components;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use crate::utils::despawn_all;
use components::OnPauseMenuScreen;
use systems::{handle_pause_input, handle_resume_input, pause_menu_action, spawn_pause_menu};
use crate::styles::menu::standard_button_system;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), despawn_all::<OnPauseMenuScreen>)
            .add_systems(
                Update,
                (standard_button_system, pause_menu_action, handle_resume_input)
                    .run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(GameState::Playing)),
            );
    }
}
