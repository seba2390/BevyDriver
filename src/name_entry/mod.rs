pub mod components;
pub mod constants;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use crate::utils::despawn_all;
use components::OnNameEntryScreen;
use systems::{cleanup_name_entry, handle_name_input, name_entry_action, spawn_name_entry};
use crate::styles::menu::standard_button_system;

pub struct NameEntryPlugin;

impl Plugin for NameEntryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::NewGameNameEntry), spawn_name_entry)
            .add_systems(OnExit(GameState::NewGameNameEntry), (despawn_all::<OnNameEntryScreen>, cleanup_name_entry))
            .add_systems(
                Update,
                (handle_name_input, standard_button_system, name_entry_action)
                    .run_if(in_state(GameState::NewGameNameEntry)),
            );
    }
}
