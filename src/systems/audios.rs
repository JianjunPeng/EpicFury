use bevy::prelude::*;
use crate::components::BackgroundMusic;

pub fn stop_background_music(
    mut commands: Commands,
    background_music_query: Query<Entity, With<BackgroundMusic>>,
) {
    for entity in &background_music_query {
        commands.entity(entity).despawn();
    }
}
