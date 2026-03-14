use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Velocity {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct ShootTimer(pub Timer);

#[derive(Component)]
pub struct Explosion(pub Timer);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct BackgroundMusic;

