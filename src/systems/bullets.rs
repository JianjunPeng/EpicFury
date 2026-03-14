use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::*;



pub fn bullet_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let speed = 600.0;
    let delta = time.delta_secs();
    if let Ok(window) = window_query.single() {
        let half_height = window.height() / 2.0;
        let margin = 50.0;
        for (entity, mut transform) in &mut query {
            transform.translation.y += speed * delta;
            if transform.translation.y > half_height + margin {
                commands.entity(entity).despawn();
            }
        }
    }
}


