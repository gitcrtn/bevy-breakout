use bevy::prelude::*;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}