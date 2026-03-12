use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Velocity {
    direction: Vec2,
}

#[derive(Component)]
struct ShootTimer(Timer);

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Explosion(Timer);

#[derive(Resource, Default)]
struct GameOver(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement,
            player_shoot,
            bullet_movement,
            enemy_movement,
            spawn_enemies,
            player_enemy_collision,
            bullet_enemy_collision,
            update_score_ui,
            despawn_explosions,  // 已添加
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    // 分数 UI（独立实体）
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout {
            justify: Justify::Left,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        GlobalTransform::default(),
        Visibility::Visible,
        ViewVisibility::default(),
        ScoreText,
    ));

    // 玩家飞机（独立实体）
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        GlobalTransform::default(),
        Visibility::Visible,
        ViewVisibility::default(),
        Player,
        ShootTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
    ));

    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(1.2, TimerMode::Repeating)));
    commands.insert_resource(Score(0));
    commands.insert_resource(GameOver(false));
}

fn player_movement(
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

fn player_shoot(
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

fn bullet_movement(
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

fn spawn_enemies(
    time: Res<Time>,
    game_over: ResMut<GameOver>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut commands: Commands,
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
                    color: Color::srgb(0.0, 0.8, 0.3),
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

fn enemy_movement(
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

fn update_score_ui(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("Score: {}", score.0);
    }
}

fn bullet_enemy_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullet_query: Query<(Entity, &Transform, &Sprite), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform, bullet_sprite) in &bullet_query {
        for (enemy_entity, enemy_transform, enemy_sprite) in &enemy_query {
            if let (Some(b_size), Some(e_size)) = (bullet_sprite.custom_size, enemy_sprite.custom_size) {
                let b_min = bullet_transform.translation.truncate() - b_size / 2.0;
                let b_max = bullet_transform.translation.truncate() + b_size / 2.0;
                let e_min = enemy_transform.translation.truncate() - e_size / 2.0;
                let e_max = enemy_transform.translation.truncate() + e_size / 2.0;

                if b_min.x < e_max.x && b_max.x > e_min.x &&
                   b_min.y < e_max.y && b_max.y > e_min.y {
                    commands.entity(bullet_entity).despawn();
                    commands.entity(enemy_entity).despawn();
                    score.0 += 10;

                    // 圆形爆炸（用 Mesh + Circle）
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(1.0, 0.5, 0.0),
                            custom_size: Some(Vec2::new(60.0, 60.0)),
                            ..default()
                        },
                        // 注意：这里仍用 Sprite，但实际想圆形需 Mesh2dHandle（见下方说明）
                        Transform::from_translation(enemy_transform.translation),
                        GlobalTransform::default(),
                        Visibility::Visible,
                        ViewVisibility::default(),
                        Explosion(Timer::from_seconds(0.3, TimerMode::Once)),
                    ));
                }
            }
        }
    }
}

fn despawn_explosions(
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

fn player_enemy_collision(
    mut commands: Commands,
    mut game_over: ResMut<GameOver>,
    player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if game_over.0 { return; }  // 已结束就不检测了

    if let Ok((player_entity, player_transform, player_sprite)) = player_query.single() {
        if let Some(p_size) = player_sprite.custom_size {
            let p_min = player_transform.translation.truncate() - p_size / 2.0;
            let p_max = player_transform.translation.truncate() + p_size / 2.0;

            for (enemy_entity, enemy_transform, enemy_sprite) in &enemy_query {
                if let Some(e_size) = enemy_sprite.custom_size {
                    let e_min = enemy_transform.translation.truncate() - e_size / 2.0;
                    let e_max = enemy_transform.translation.truncate() + e_size / 2.0;

                    if p_min.x < e_max.x && p_max.x > e_min.x &&
                       p_min.y < e_max.y && p_max.y > e_min.y {
                        // 碰撞！游戏结束
                        game_over.0 = true;

                        // despawn 玩家和这个敌人
                        commands.entity(player_entity).despawn();
                        commands.entity(enemy_entity).despawn();

                        // 显示 Game Over 文本（居中）
                        if let Ok(_window) = window_query.single() {
                            // 显示 Game Over 文本（尽量居中）
                            commands.spawn((
                                Text::new("GAME OVER"),
                                TextFont {
                                    font_size: 80.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.2, 0.2)),
                                TextLayout {
                                    justify: Justify::Center,
                                    ..default()
                                },
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(50.0),
                                    top: Val::Percent(50.0),
                                    margin: UiRect {
                                        left: Val::Px(-200.0),  // 往左移 200px（负值左移）
                                        ..default()
                                    },
                                    ..default()
                                },
                                GlobalTransform::default(),
                                Visibility::Visible,
                                ViewVisibility::default(),
                            ));
                        }

                        // 可以在这里加重启逻辑（后面再扩展）
                        break;  // 只处理一次碰撞
                    }
                }
            }
        }
    }
}
