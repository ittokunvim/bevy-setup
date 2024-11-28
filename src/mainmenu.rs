use bevy::prelude::*;

use crate::{
    GAMETITLE,
    WINDOW_SIZE,
    PATH_FONT,
};

const GAMETITLE_FONT_SIZE: f32 = 32.0;
const GAMETITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

#[derive(Default, Component, Debug)]
struct Mainmenu;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("mainmenu: setup gametitle");
    commands.spawn((
        TextBundle::from_section(
            GAMETITLE,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: GAMETITLE_FONT_SIZE,
                color: GAMETITLE_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_FONT_SIZE / 2.0),
            ..Default::default()
        }),
        Mainmenu,
    ))
    .insert(Name::new("gametitle"));
}

pub struct MainmenuPlugin;

impl Plugin for MainmenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}
