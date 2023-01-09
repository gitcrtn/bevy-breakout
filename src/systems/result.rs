use bevy::prelude::*;
use crate::components::ResultText;
use crate::resources::{Game, GameStatus};

pub fn update_result(game: Res<Game>, mut query: Query<&mut Text, With<ResultText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = match game.status {
        GameStatus::Won => "You Win".to_string(),
        GameStatus::GameOver => "Game Over".to_string(),
        _ => "".to_string(),
    };
}