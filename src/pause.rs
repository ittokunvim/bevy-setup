use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_IMAGE_PAUSEBUTTON,
    AppState,
};

const IMAGE_SIZE: u32 = 64;
const SIZE: f32 = 32.0;

#[derive(Default, Component, Debug)]
struct PauseButton {
    first: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("pause: setup pausebutton");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(IMAGE_SIZE), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = PauseButton { first: 0 };
    let pos = Vec3::new(
        WINDOW_SIZE.x / 2.0 - SIZE,
        -WINDOW_SIZE.y / 2.0 + SIZE,
        10.0
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SIZE)),
                ..Default::default()
            },
            texture: asset_server.load(PATH_IMAGE_PAUSEBUTTON),
            transform: Transform::from_translation(pos),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
    ))
    .insert(Name::new("pausebutton"));
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup);
    }
}
