pub mod components;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use crate::utils::despawn_all;
use components::OnMenuScreen;
use systems::{menu_action, spawn_menu};
use crate::styles::menu::standard_button_system;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::StartMenu), spawn_menu)
            .add_systems(OnExit(GameState::StartMenu), despawn_all::<OnMenuScreen>)
            .add_systems(
                Update,
                (standard_button_system, menu_action).run_if(in_state(GameState::StartMenu)),
            );
    }
}
