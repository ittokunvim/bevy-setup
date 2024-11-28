use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    AppState,
    Config,
};
use crate::pause::PauseButton;

const INGAME_TEXT: &str = "Ingame";
const INGAME_FONT_SIZE: f32 = 32.0;
const GAMEOVER_TEXT: &str = "Gameover: Key[A]";
const GAMECLEAR_TEXT: &str = "Gameclear: Key[D]";
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_FONT_SIZE: f32 = 20.0;
const TEXT_PADDING: f32 = 40.0;

#[derive(Default, Component, Debug)]
struct Ingame;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: &mut Config,
) {
    if config.setup { return }
    config.setup = true;

    println!("ingame: setup ingame text");
    commands.spawn((
        TextBundle::from_section(
            INGAME_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: INGAME_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - INGAME_FONT_SIZE / 2.0 - TEXT_PADDING),
            ..Default::default()
        }),
        Ingame,
    ))
    .insert(Name::new("ingame"));
    println!("ingame: setup gameover text");
    commands.spawn((
        TextBundle::from_section(
            GAMEOVER_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_FONT_SIZE / 2.0),
            ..Default::default()
        }),
        Ingame,
    ))
    .insert(Name::new("ingame"));
    println!("ingame: setup gameclear text");
    commands.spawn((
        TextBundle::from_section(
            GAMECLEAR_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - INGAME_FONT_SIZE / 2.0 + TEXT_PADDING),
            ..Default::default()
        }),
        Ingame,
    ))
    .insert(Name::new("ingame"));
}

fn update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ingame_query: Query<Entity, With<Ingame>>,
    pausebutton_query: Query<Entity, With<PauseButton>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let mut closure = |key: &KeyCode, next_state: AppState| {
        println!("ingame: {:?} just pressed", key);
        println!("ingame: despawned entities");
        for entity in ingame_query.iter() {
            commands.entity(entity).despawn();
        }
        commands.entity(pausebutton_query.single()).despawn();
        println!("ingame: moved Ingame -> {:?}", next_state);
        app_state.set(next_state);
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyA => closure(key, AppState::Gameover),
            KeyCode::KeyD => closure(key, AppState::Gameclear),
            _ => {},
        }
    }
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        let mut config = Config { setup: false, };

        app
            .add_systems(
                OnEnter(AppState::Ingame), move |
                commands: Commands,
                asset_server: Res<AssetServer>,
                | { setup(commands, asset_server, &mut config); }
            )
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)));
    }
}
