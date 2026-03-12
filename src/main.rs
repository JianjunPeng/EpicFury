use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, (
            player_movement,
            player_shoot,
            bullet_movement,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    // 2D 相机：现在直接 spawn Camera2d（其他组件自动补齐）
    commands.spawn(Camera2d::default());

    // 玩家飞机：用 Sprite + Transform + Visibility 等
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),  // 蓝色方块
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
    mut query: Query<(&Velocity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    let speed = 600.0;  // 子弹速度（比飞机快很多）
    let delta = time.delta_secs();

    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.direction.x * speed * delta;
        transform.translation.y += velocity.direction.y * speed * delta;
    }
}
