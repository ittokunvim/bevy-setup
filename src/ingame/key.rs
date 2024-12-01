use bevy::prelude::*;

use crate::AppState;

fn update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut closure = |key: &KeyCode, app_state: AppState| {
        println!("key: {:?} just pressed", key);
        println!("key: moved state to {:?} from Ingame", app_state);
        next_state.set(app_state);
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyA => closure(key, AppState::Gameover),
            KeyCode::KeyD => closure(key, AppState::Gameclear),
            _ => {},
        }
    }
}

pub struct KeyPlugin;

impl Plugin for KeyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)));
    }
}
