use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::components::*;
use crate::resources::*;



pub fn spawn_enemies(
    time: Res<Time>,
    game_over: ResMut<GameOver>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if game_over.0 { return; }

    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if let Ok(window) = window_query.single() {
            let half_width = window.width() / 2.0;
            let spawn_x = rand::thread_rng().gen_range(-half_width + 30.0..half_width - 30.0);
            commands.spawn((
                Sprite {
                    image: asset_server.load("images/enemy.png"),
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                Transform::from_xyz(spawn_x, window.height() / 2.0 + 50.0, 0.0),
                GlobalTransform::default(),
                Visibility::Visible,
                ViewVisibility::default(),
                Velocity { direction: Vec2::new(0.0, -1.0) },
                Enemy,
            ));
        }
    }
}

pub fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Enemy>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let speed = 180.0;
    let delta = time.delta_secs();
    if let Ok(window) = window_query.single() {
        let half_height = window.height() / 2.0;
        let margin = 50.0;
        for (entity, mut transform) in &mut query {
            transform.translation.y -= speed * delta;
            if transform.translation.y < -half_height - margin {
                commands.entity(entity).despawn();
            }
        }
    }
}

