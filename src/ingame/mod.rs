use bevy::prelude::*;

mod key;
mod pausebutton;
mod text;

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(key::KeyPlugin)
            .add_plugins(pausebutton::PausebuttonPlugin)
            .add_plugins(text::TextPlugin);
    }
}
