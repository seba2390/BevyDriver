pub mod components;
pub mod constants;
pub mod helpers;
pub mod systems;
pub mod track_generator;
pub mod tracks;

use bevy::prelude::*;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, _app: &mut App) {
        // Road systems are currently orchestrated in GamePlugin due to strict ordering requirements
    }
}
