use bevy::prelude::*;
use crate::components::ScoreboardText;
use crate::resources::Scoreboard;

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardText>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}