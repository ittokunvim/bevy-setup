use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    AppState,
    Config,
    Score,
    GameTimer,
};

const GAMECLEAR_TEXT: &str = "ゲームクリア";
const GAMECLEAR_FONT_SIZE: f32 = 32.0;
const SCORE_TEXT: &str = "スコア: ";
const TIMER_TEXT: &str = "タイム: ";
const RETRY_TEXT: &str = "リトライ: Key[R]";
const BACKTOTITLE_TEXT: &str = "タイトルに戻る: Key[B]";
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_FONT_SIZE: f32 = 20.0;
const TEXT_PADDING: f32 = 40.0;

#[derive(Component)]
struct Gameclear;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    timer: Res<GameTimer>,
) {
    println!("gameclear: setup");
    // gameover
    let top = WINDOW_SIZE.y / 2.0 - GAMECLEAR_FONT_SIZE / 2.0 - TEXT_PADDING * 2.0;

    commands.spawn((
        TextBundle::from_section(
            GAMECLEAR_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: GAMECLEAR_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("gameclear"));
    // score
    let top = WINDOW_SIZE.y / 2.0 - TEXT_FONT_SIZE / 2.0 - TEXT_PADDING;

    commands.spawn((
        TextBundle::from_section(
            format!("{}{}", SCORE_TEXT, **score), 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("score"));
    // timer
    let top = WINDOW_SIZE.y / 2.0 - TEXT_FONT_SIZE / 2.0;

    commands.spawn((
        TextBundle::from_section(
            format!("{}{}", TIMER_TEXT, timer.0.remaining_secs().round().to_string()),
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("timer"));
    // retry
    let top = WINDOW_SIZE.y / 2.0 - TEXT_FONT_SIZE / 2.0 + TEXT_PADDING;

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
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("retry"));
    // back to title
    let top = WINDOW_SIZE.y / 2.0 - TEXT_FONT_SIZE / 2.0 + TEXT_PADDING * 2.0;

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
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("backtotitle"));
}

fn update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut closure = |key: &KeyCode, app_state: AppState| {
        println!("gameclear: {:?} just pressed", key);
        println!("gameclear: config setup ingame is true");
        config.setup_ingame = true;
        println!("gameclear: moved state to {:?} from Gameclear", app_state);
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

fn despawn_gameclear(
    mut commands: Commands,
    query: Query<Entity, With<Gameclear>>,
) {
    println!("gameclear: despawned");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct GameclearPlugin;

impl Plugin for GameclearPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameclear), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameclear)))
            .add_systems(OnExit(AppState::Gameclear), despawn_gameclear);
    }
}
