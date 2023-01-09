use bevy::prelude::*;
use crate::entities::ball::spawn_ball;
use crate::entities::brick::spawn_bricks;
use crate::entities::paddle::spawn_paddle;
use crate::entities::scoreboard::spawn_scoreboard;
use crate::entities::wall::spawn_walls;
use crate::sounds::setup_sounds;

// Add the game's entities to our world
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    setup_sounds(&mut commands, &asset_server);

    // Paddle
    spawn_paddle(&mut commands);

    // Ball
    spawn_ball(&mut commands, &mut meshes, &mut materials);

    // Scoreboard
    spawn_scoreboard(&mut commands, &asset_server);

    // Walls
    spawn_walls(&mut commands);

    // Bricks
    spawn_bricks(&mut commands);
}