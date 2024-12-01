use bevy::prelude::*;

use crate::{
    PATH_FONT,
    AppState,
    Config,
    Score,
};

const SCORE_TEXT: &str = "スコア: ";
const TEXT_FONT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Component)]
struct ScoreboardUi;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                SCORE_TEXT,
                TextStyle {
                    font: asset_server.load(PATH_FONT),
                    font_size: TEXT_FONT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: TEXT_PADDING,
            left: TEXT_PADDING,
            ..Default::default()
        }),
        ScoreboardUi,
    ));
}

fn update(
    mut query: Query<&mut Text, With<ScoreboardUi>>,
    score: Res<Score>,
) {
    let mut text = query.single_mut();
    // write score
    text.sections[1].value = score.to_string();
}

fn despawn_scoreboard(
    mut commands: Commands,
    query: Query<Entity, With<ScoreboardUi>>,
) {
    let entity = query.single();
    println!("scoreboard: despawned");
    commands.entity(entity).despawn();
}

fn score_points(mut score: ResMut<Score>) {
    if **score >= 500 { **score = 0; }

    **score += 1;
}

fn reset_score(mut score: ResMut<Score>) {
    **score = 0;
}

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, score_points.run_if(in_state(AppState::Ingame)))
            .add_systems(OnEnter(AppState::Gameover), despawn_scoreboard)
            .add_systems(OnExit(AppState::Gameover), reset_score);
    }
}
