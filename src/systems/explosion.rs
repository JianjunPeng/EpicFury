use bevy::prelude::*;
use crate::components::*;


pub fn despawn_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion)>,
) {
    for (entity, mut explosion) in &mut query {
        explosion.0.tick(time.delta());
        if explosion.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
