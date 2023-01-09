use bevy::prelude::*;
use crate::resources::CollisionSound;

pub fn setup_sounds(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));
}