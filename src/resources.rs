use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource, Default)]
pub struct GameOver(pub bool);

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource, Clone)]
pub struct GameSounds {
    pub shoot: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
    pub bgm: Handle<AudioSource>,
}
