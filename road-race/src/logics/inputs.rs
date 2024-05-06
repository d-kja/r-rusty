use crate::*;

pub fn control_inputs(engine: &mut Engine, state: &mut GameState) {
    let key_events = &engine.keyboard_state;

    match key_events {
        key if key.just_pressed(KeyCode::S) => {
            let player_one = state.players.get_mut(0).unwrap();

            if player_one.direction < 0.0 {
                return;
            }

            player_one.direction -= 1.0;
        }
        key if key.just_pressed(KeyCode::W) => {
            let player_one = state.players.get_mut(0).unwrap();

            if player_one.direction > 0.0 {
                return;
            }

            player_one.direction += 1.0;
        }

        key if key.just_pressed(KeyCode::Down) => {
            let player_two = state.players.get_mut(1).unwrap();

            if player_two.direction < 0.0 {
                return;
            }

            player_two.direction -= 1.0;
        }
        key if key.just_pressed(KeyCode::Up) => {
            let player_two = state.players.get_mut(1).unwrap();

            if player_two.direction > 0.0 {
                return;
            }

            player_two.direction += 1.0;
        }
        _ => (),
    }
}
