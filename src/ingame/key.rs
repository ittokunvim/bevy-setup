use bevy::prelude::*;

use crate::AppState;

fn update(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut closure = |app_state: AppState| {
        println!("key: moved state to {:?} from Ingame", app_state);
        next_state.set(app_state);
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyA => closure(AppState::Gameover),
            KeyCode::KeyD => closure(AppState::Gameclear),
            _ => {},
        }
    }
}

pub struct KeyPlugin;

impl Plugin for KeyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
        ;
    }
}
