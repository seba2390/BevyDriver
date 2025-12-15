use bevy::prelude::*;
use crate::start_menu::components::GameEntity;

/// Generic despawn system that removes all entities with the specified marker component.
/// This can be used with any marker component to clean up entities when leaving a state.
///
/// # Example Usage in main.rs:
/// ```rust
/// .add_systems(OnExit(GameState::StartMenu), despawn_all::<OnMenuScreen>)
/// ```
pub fn despawn_all<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Generic helper to spawn a HUD text element
pub fn spawn_hud_element<B: Bundle, M: Component>(
    commands: &mut Commands,
    text: String,
    style_bundle: B,
    marker: M,
    visibility: Visibility,
) {
    commands.spawn((
        Text::new(text),
        style_bundle,
        visibility,
        marker,
        GameEntity,
    ));
}
