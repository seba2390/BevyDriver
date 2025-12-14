use bevy::prelude::*;

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
