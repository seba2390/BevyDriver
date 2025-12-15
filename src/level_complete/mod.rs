pub mod components;
pub mod constants;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use crate::utils::despawn_all;
use crate::start_menu::components::GameEntity;
use components::OnLevelCompleteScreen;
use systems::{level_complete_action, spawn_level_complete_menu};
use crate::styles::menu::standard_button_system;

pub struct LevelCompletePlugin;

impl Plugin for LevelCompletePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LevelComplete), spawn_level_complete_menu)
            .add_systems(OnExit(GameState::LevelComplete), (despawn_all::<OnLevelCompleteScreen>, despawn_all::<GameEntity>))
            .add_systems(
                Update,
                (standard_button_system, level_complete_action)
                    .run_if(in_state(GameState::LevelComplete)),
            );
    }
}
