use bevy::prelude::*;
use crate::components::ResultText;
use crate::constants::*;

pub fn spawn_result(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(
       NodeBundle {
           style: Style {
               size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
               align_items: AlignItems::Center,
               justify_content: JustifyContent::Center,
               ..Default::default()
           },
           ..Default::default()
       })
       .with_children(|parent| {
           parent.spawn((
               TextBundle::from_sections([
                   TextSection::new(
                       "",
                       TextStyle {
                           font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                           font_size: RESULT_FONT_SIZE,
                           color: RESULT_COLOR,
                       },
                   ),
               ]),
               ResultText
           ));
       });
}