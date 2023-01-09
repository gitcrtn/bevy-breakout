use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::components::{Ball, Velocity};
use crate::constants::*;

pub fn spawn_ball(commands: &mut Commands,
                  meshes: &mut ResMut<Assets<Mesh>>,
                  materials: &mut ResMut<Assets<ColorMaterial>>,) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));
}