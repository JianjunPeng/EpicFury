use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::*;
use crate::resources::*;



pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_over: ResMut<GameOver>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if game_over.0 { return; }

    let speed = 500.0;
    let delta = time.delta_secs();
    let player_half_size = 25.0;

    for mut transform in &mut query {
        let mut direction = Vec2::ZERO;
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation.x += direction.x * speed * delta;
            transform.translation.y += direction.y * speed * delta;
        }

        if let Ok(window) = window_query.single() {
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;
            transform.translation.x = transform.translation.x.clamp(
                -half_width + player_half_size,
                half_width - player_half_size,
            );
            transform.translation.y = transform.translation.y.clamp(
                -half_height + player_half_size,
                half_height - player_half_size,
            );
        }
    }
}

pub fn player_shoot(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_over: ResMut<GameOver>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut ShootTimer), With<Player>>,
) {
    if game_over.0 { return; }

    if let Ok((player_transform, mut shoot_timer)) = query.single_mut() {
        shoot_timer.0.tick(time.delta());
        if keyboard.pressed(KeyCode::Space) && shoot_timer.0.is_finished() {
            let spawn_pos = player_transform.translation + Vec3::new(0.0, 30.0, 0.0);
            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 0.3, 0.3),
                    custom_size: Some(Vec2::new(8.0, 24.0)),
                    ..default()
                },
                Transform::from_translation(spawn_pos),
                GlobalTransform::default(),
                Visibility::Visible,
                ViewVisibility::default(),
                Velocity { direction: Vec2::new(0.0, 1.0) },
                Bullet,
            ));
            shoot_timer.0.reset();
        }
    }
}


