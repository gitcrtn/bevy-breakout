use bevy::prelude::*;
use crate::components::Brick;
use crate::resources::{Game, GameStatus};

pub fn update_game(
    mut game: ResMut<Game>,
    brick_query: Query<Entity, With<Brick>>,
) {
    if brick_query.is_empty() {
        game.status = GameStatus::Won;
    }
}