use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    AppState,
    Config,
};

const GAMEOVER_TEXT: &str = "Game Over";
const GAMEOVER_FONT_SIZE: f32 = 32.0;
const RETRY_TEXT: &str = "Retry: Key[R]";
const BACKTOTITLE_TEXT: &str = "Back To Title: Key[B]";
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_FONT_SIZE: f32 = 20.0;
const TEXT_PADDING: f32 = 40.0;

#[derive(Component)]
struct Gameover;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("gameover: setup");
    // gameover
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - GAMEOVER_FONT_SIZE / 2.0 - TEXT_PADDING);

    commands.spawn((
        TextBundle::from_section(
            GAMEOVER_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: GAMEOVER_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
                position_type: PositionType::Relative,
                justify_self: JustifySelf::Center,
                top,
                ..Default::default()
            }),
        Gameover,
    ))
    .insert(Name::new("gameover"));
    // retry
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_FONT_SIZE / 2.0);

    commands.spawn((
        TextBundle::from_section(
            RETRY_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
                position_type: PositionType::Relative,
                justify_self: JustifySelf::Center,
                top,
                ..Default::default()
            }),
        Gameover,
    ))
    .insert(Name::new("retry"));
    // back to title
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_FONT_SIZE / 2.0 + TEXT_PADDING);

    commands.spawn((
        TextBundle::from_section(
            BACKTOTITLE_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
                position_type: PositionType::Relative,
                justify_self: JustifySelf::Center,
                top,
                ..Default::default()
            }),
        Gameover,
    ))
    .insert(Name::new("backtotitle"));
}

fn update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut closure = |key: &KeyCode, app_state: AppState| {
        println!("gameover: {:?} just pressed", key);
        println!("gameover: config setup ingame is true");
        config.setup_ingame = true;
        println!("gameover: moved state to {:?} from Gameover", app_state);
        next_state.set(app_state);
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyR => closure(key, AppState::Ingame),
            KeyCode::KeyB => closure(key, AppState::Mainmenu),
            _ => {},
        }
    }
}

fn despawn_gameover(
    mut commands: Commands,
    query: Query<Entity, With<Gameover>>,
) {
    println!("gameover: despawned");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)))
            .add_systems(OnExit(AppState::Gameover), despawn_gameover);
    }
}
