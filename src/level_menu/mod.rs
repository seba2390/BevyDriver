pub mod components;
pub mod constants;
pub mod minimap;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use crate::utils::despawn_all;
use components::OnLevelMenuScreen;
use minimap::{capture_minimaps, cleanup_minimap_rendering, setup_minimap_rendering, MinimapCache};
use systems::{level_menu_action, spawn_level_menu, update_minimap_previews};
use crate::styles::menu::standard_button_system;

pub struct LevelMenuPlugin;

impl Plugin for LevelMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MinimapCache>()
            .add_systems(OnEnter(GameState::LevelMenu), (spawn_level_menu, setup_minimap_rendering).chain())
            .add_systems(OnExit(GameState::LevelMenu), (despawn_all::<OnLevelMenuScreen>, cleanup_minimap_rendering))
            .add_systems(
                Update,
                (
                    standard_button_system,
                    level_menu_action,
                    capture_minimaps,
                    update_minimap_previews,
                )
                    .run_if(in_state(GameState::LevelMenu)),
            );
    }
}
