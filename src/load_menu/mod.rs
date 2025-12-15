pub mod components;
pub mod constants;
pub mod systems;

use bevy::prelude::*;
use crate::constants::GameState;
use crate::utils::despawn_all;
use components::OnLoadMenuScreen;
use systems::{
    cleanup_load_menu, handle_delete_click, handle_delete_confirm_action, handle_save_slot_click,
    load_menu_action, spawn_load_menu,
};
use crate::styles::menu::standard_button_system;

pub struct LoadMenuPlugin;

impl Plugin for LoadMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LoadGameMenu), spawn_load_menu)
            .add_systems(OnExit(GameState::LoadGameMenu), (despawn_all::<OnLoadMenuScreen>, cleanup_load_menu))
            .add_systems(
                Update,
                (
                    standard_button_system,
                    handle_save_slot_click,
                    handle_delete_click,
                    handle_delete_confirm_action,
                    load_menu_action,
                )
                    .run_if(in_state(GameState::LoadGameMenu)),
            );
    }
}
