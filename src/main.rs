use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Velocity {
    direction: Vec2,  // 单位向量 × 速度
}

#[derive(Component)]
struct ShootTimer(Timer);  // 射击冷却计时器

// = = = = = = = = = = = Enemy
#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct EnemySpawnTimer(Timer);


// = = = = = = = = = = = Score
#[derive(Resource, Default)]
struct Score(u32);

#[derive(Component)]
struct ScoreText;


// = = = = = = = = = = = Explosion
#[derive(Component)]
struct Explosion(Timer);



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement,
            player_shoot,
            bullet_movement,
            enemy_movement,          // 新增
            bullet_enemy_collision,
            update_score_ui,
            spawn_enemies,           // 新增
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");  // 放 assets/fonts 下

    // 2D 相机：现在直接 spawn Camera2d（其他组件自动补齐）
    commands.spawn(Camera2d::default());

    // 玩家飞机：用 Sprite + Transform + Visibility 等
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font: font_handle,
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),

        // 玩家蓝色方块
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),  // 初始位置
        GlobalTransform::default(),             // 必须加（渲染需要）
        Visibility::Visible,                    // 或用 InheritedVisibility 等
        ViewVisibility::default(),              // 渲染可见性（旧版自动带的）
        Player,
        ShootTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),  // 每 0.15 秒可射一次
    ));

    // 得分记录
    commands.spawn((
        Text::new("Score: 0"),  // 现在 Text::new 直接接受字符串
        TextFont {
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
        // 渲染必须的组件（UI 文本需要）
        GlobalTransform::default(),
        Visibility::Visible,
        ViewVisibility::default(),
        ScoreText,
    ));
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(1.2, TimerMode::Repeating))); // 每 1.2 秒产生敌人
    commands.insert_resource(Score(0));
}

// 0. 玩家移动系统（使用上下左右键触发）
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,  // 新增：获取窗口尺寸
) {
    let speed = 300.0;
    let delta = time.delta_secs();
    let player_half_size = 25.0;  // 你的精灵是 50x50，所以半宽半高是 25（防止边缘卡住）

    for mut transform in &mut query {
        let mut direction = Vec2::ZERO;

        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation.x += direction.x * speed * delta;
            transform.translation.y += direction.y * speed * delta;
        }

        // === 新增：边界限制 ===
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

// 1. 射击系统（只在按下空格时触发一次）
fn player_shoot(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut ShootTimer), With<Player>>,
) {
    if let Ok((player_transform, mut shoot_timer)) = query.single_mut() {
        // 计时器每帧 tick
        shoot_timer.0.tick(time.delta());

        // 按住空格 且 冷却结束 → 射击
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

            // 重置计时器（从 0.15 秒开始倒计时）
            shoot_timer.0.reset();
        }
    }
}

// 2. 子弹移动系统
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
        let margin = 50.0;  // 和敌人保持一致

        for (entity, mut transform) in &mut query {
            transform.translation.y += speed * delta;  // 子弹向上

            // 飞出顶部就销毁
            if transform.translation.y > half_height + margin {
                commands.entity(entity).despawn();
            }
        }
    }
}

// 1. 敌人生成系统（随机 x 位置）
fn spawn_enemies(
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        if let Ok(window) = window_query.single() {
            let half_width = window.width() / 2.0;
            let spawn_x = rand::thread_rng().gen_range(-half_width + 30.0..half_width - 30.0);

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.0, 0.8, 0.3),  // 绿色敌人
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                Transform::from_xyz(spawn_x, window.height() / 2.0 + 50.0, 0.0),  // 从顶部上方生成
                GlobalTransform::default(),
                Visibility::Visible,
                ViewVisibility::default(),
                Velocity { direction: Vec2::new(0.0, -1.0) },
                Enemy,
            ));
        }
    }
}

// 2. 敌人移动系统（向下掉）
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
        let margin = 50.0;  // 飞出一点再删，避免边缘抖动

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
        text.0 = format!("Score: {}", score.0);  // 直接改内部 String
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

                // AABB 重叠检查（经典算法）
                if b_min.x < e_max.x && b_max.x > e_min.x &&
                   b_min.y < e_max.y && b_max.y > e_min.y {
                    // 碰撞！销毁双方
                    commands.entity(bullet_entity).despawn();
                    commands.entity(enemy_entity).despawn();

                    // 加分
                    score.0 += 10;  // 可调

                    // Explosion
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(1.0, 0.5, 0.0),  // 橙色爆炸
                            custom_size: Some(Vec2::new(60.0, 60.0)),
                            ..default()
                        },
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
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
