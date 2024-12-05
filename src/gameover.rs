use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    AppState,
    Config,
    Score,
};

const GAMEOVER_TEXT: &str = "ゲームオーバー";
const GAMEOVER_SIZE: f32 = 32.0;
const SCORE_TEXT: &str = "スコア: ";
const RETRY_TEXT: &str = "リトライ: Key[R]";
const BACKTOTITLE_TEXT: &str = "タイトルに戻る: Key[B]";
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_SIZE: f32 = 20.0;
const TEXT_PADDING: f32 = 50.0;

#[derive(Component)]
struct Gameover;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    println!("gameover: setup");
    // gameover
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - GAMEOVER_SIZE / 2.0 - TEXT_PADDING * 1.5);

    commands.spawn((
        TextBundle::from_section(
            GAMEOVER_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: GAMEOVER_SIZE,
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
    // score
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 - TEXT_PADDING * 0.5);

    commands.spawn((
        TextBundle::from_section(
            format!("{}{}", SCORE_TEXT, **score), 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
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
    .insert(Name::new("score"));
    // retry
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + TEXT_PADDING * 0.5);

    commands.spawn((
        TextBundle::from_section(
            RETRY_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
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
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + TEXT_PADDING * 1.5);

    commands.spawn((
        TextBundle::from_section(
            BACKTOTITLE_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
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
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut closure = |app_state: AppState| {
        println!("gameover: config setup ingame is true");
        config.setup_ingame = true;
        println!("gameover: moved state to {:?} from Gameover", app_state);
        next_state.set(app_state);
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyR => closure(AppState::Ingame),
            KeyCode::KeyB => closure(AppState::Mainmenu),
            _ => {},
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Gameover>>,
) {
    println!("gameover: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)))
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}
