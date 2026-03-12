use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
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
    ));
}

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
