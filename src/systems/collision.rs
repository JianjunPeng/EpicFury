use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::*;
use crate::resources::*;



pub fn bullet_enemy_collision(
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



pub fn player_enemy_collision(
    mut commands: Commands,
    mut game_over: ResMut<GameOver>,
    background_music_query: Query<Entity, With<BackgroundMusic>>,
    player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
    _window_query: Query<&Window, With<PrimaryWindow>>,
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

                        // 停止 BGM
                        for entity in &background_music_query {
                            commands.entity(entity).despawn();
                        }

                        // 显示 Game Over 文本（居中）
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
                                    left: Val::Px(-200.0),
                                    ..default()
                                },
                                ..default()
                            },
                            GlobalTransform::default(),
                            Visibility::Visible,
                            ViewVisibility::default(),
                        ));

                        // 可以在这里加重启逻辑（后面再扩展）
                        break;  // 只处理一次碰撞
                    }
                }
            }
        }
    }
}
