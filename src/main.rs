use bevy::prelude::*;
use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};

mod components;
mod resources;
mod systems;

use components::{BackgroundMusic, Player, ScoreText, ShootTimer};
use resources::{EnemySpawnTimer, GameOver, GameSounds, Score};
use systems::bullets;
use systems::collision;
use systems::explosion;
use systems::enemies;
use systems::players;
use systems::ui;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            players::player_movement,
            players::player_shoot,
            bullets::bullet_movement,
            enemies::enemy_movement,
            enemies::spawn_enemies,
            collision::player_enemy_collision,
            collision::bullet_enemy_collision,
            explosion::despawn_explosions,
            ui::update_score_ui,
        ))
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
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

    // 加载音效 + BGM
    let game_sounds = GameSounds {
        shoot: asset_server.load("sounds/shoot.ogg"),
        explosion: asset_server.load("sounds/explosion.ogg"),
        bgm: asset_server.load("sounds/background.ogg"),
    };
    commands.insert_resource(game_sounds.clone());

    commands.spawn((
        AudioPlayer(game_sounds.bgm.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.25)),
        BackgroundMusic,
    ));
}
