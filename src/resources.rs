use bevy::prelude::*;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

#[warn(dead_code)]
pub enum GameStatus {
    Playing,
    GameOver,
    Won,
}

#[derive(Resource)]
pub struct Game {
    pub status: GameStatus,
}