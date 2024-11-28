use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    AppState,
};

const INGAME_FONT_SIZE: f32 = 32.0;
const INGAME_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

#[derive(Default, Component, Debug)]
struct Ingame;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("ingame: setup text");
    commands.spawn((
        TextBundle::from_section(
            "Ingame",
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: INGAME_FONT_SIZE,
                color: INGAME_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - INGAME_FONT_SIZE / 2.0),
            ..default()
        }),
        Ingame,
    ))
    .insert(Name::new("ingame"));
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup);
    }
}
