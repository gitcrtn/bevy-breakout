use bevy::prelude::*;
use crate::constants::*;
use crate::events::CollisionEvent;
use crate::resources::{Game, GameStatus, Scoreboard};
use crate::systems::game::update_game;
use crate::systems::startup::setup;
use crate::systems::paddle::create_paddle_system_set;
use crate::systems::result::update_result;
use crate::systems::scoreboard::update_scoreboard;

mod constants;
mod systems;
mod components;
mod events;
mod resources;
mod entities;
mod sounds;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(Game { status: GameStatus::Playing })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            create_paddle_system_set(),
        )
        .add_system(update_scoreboard)
        .add_system(update_game)
        .add_system(update_result)
        .add_system(bevy::window::close_on_esc)
        .run();
}