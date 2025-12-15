use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::prelude::*;
use bevy_scrollbar::ScrollbarPlugin;

mod car;
mod constants;
mod game_plugin;
mod hud;
mod level_complete;
mod level_menu;
mod load_menu;
mod name_entry;
mod pause_menu;
mod save;
mod start_menu;
mod road;
mod styles;
mod utils;

use car::CarPlugin;
use constants::{CurrentLevel, GameState, ResumeFromPause, WINDOW_HEIGHT, WINDOW_WIDTH, BLOOM_INTENSITY, GAME_BACKGROUND_COLOR};
use game_plugin::GamePlugin;
use hud::HudPlugin;
use level_complete::LevelCompletePlugin;
use level_menu::LevelMenuPlugin;
use load_menu::LoadMenuPlugin;
use name_entry::NameEntryPlugin;
use pause_menu::PauseMenuPlugin;
use road::RoadPlugin;
use save::CurrentSave;
use start_menu::StartMenuPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Driver".to_string(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            ScrollbarPlugin,
        ))
        // Initialize game state (starts in Menu by default)
        .init_state::<GameState>()
        // Initialize current level resource
        .init_resource::<CurrentLevel>()
        // Initialize current save resource
        .init_resource::<CurrentSave>()
        // Initialize resume from pause flag
        .init_resource::<ResumeFromPause>()
        // Set the clear color (background color)
        .insert_resource(ClearColor(GAME_BACKGROUND_COLOR))
        // Spawn camera once on startup (persists across states)
        .add_systems(Startup, spawn_camera)
        // Add all our plugins
        .add_plugins((
            StartMenuPlugin,
            NameEntryPlugin,
            LoadMenuPlugin,
            LevelMenuPlugin,
            GamePlugin,
            CarPlugin,
            HudPlugin,
            RoadPlugin,
            PauseMenuPlugin,
            LevelCompletePlugin,
        ))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: BLOOM_INTENSITY,
            // Use prefilter to only bloom pixels above threshold (more localized)
            prefilter: BloomPrefilter {
                threshold: 1.1,           // Only bloom pixels brighter than 1.0 (HDR)
                threshold_softness: 0.2,  // Soft transition
            },
            // Additive mode works better with prefilter for localized glow
            composite_mode: BloomCompositeMode::Additive,
            // Tighten the scatter for more localized glow
            high_pass_frequency: 0.45,
            ..default()
        },
        DebandDither::Enabled,
    ));
}
