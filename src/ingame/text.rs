use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    AppState,
    Config,
};

const INGAME_TEXT: &str = "ゲーム中";
const INGAME_SIZE: f32 = 32.0;
const GAMEOVER_TEXT: &str = "ゲームオーバー: Key[A]";
const GAMECLEAR_TEXT: &str = "ゲームクリア: Key[D]";
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_SIZE: f32 = 20.0;
const TEXT_PADDING: f32 = 50.0;

#[derive(Component)]
struct IngameText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    println!("text: setup");
    // ingame
    commands.spawn((
        TextBundle::from_section(
            INGAME_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: INGAME_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - INGAME_SIZE / 2.0 - TEXT_PADDING),
            ..Default::default()
        }),
        IngameText,
    ))
    .insert(Name::new("ingame"));
    // game over
    commands.spawn((
        TextBundle::from_section(
            GAMEOVER_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0),
            ..Default::default()
        }),
        IngameText,
    ))
    .insert(Name::new("gameover"));
    // game clear
    commands.spawn((
        TextBundle::from_section(
            GAMECLEAR_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - INGAME_SIZE / 2.0 + TEXT_PADDING),
            ..Default::default()
        }),
        IngameText,
    ))
    .insert(Name::new("gameclear"));
}

fn despawn_text(
    mut commands: Commands,
    query: Query<Entity, With<IngameText>>,
) {
    println!("text: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(OnEnter(AppState::Gameover), despawn_text)
            .add_systems(OnEnter(AppState::Gameclear), despawn_text)
        ;
    }
}
