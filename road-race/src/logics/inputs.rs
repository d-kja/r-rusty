use crate::*;

const DIRECTION_LIMITER: f32 = 0.5;

pub fn control_inputs(engine: &mut Engine, state: &mut GameState) {
    let key_events = &engine.keyboard_state;

    match key_events {
        key if key.just_pressed(KeyCode::S) => {
            let player_one = state.players.get_mut(0).unwrap();

            if player_one.direction <= -DIRECTION_LIMITER {
                return;
            }

            player_one.direction -= DIRECTION_LIMITER;
        }
        key if key.just_pressed(KeyCode::W) => {
            let player_one = state.players.get_mut(0).unwrap();

            if player_one.direction >= DIRECTION_LIMITER {
                return;
            }

            player_one.direction += DIRECTION_LIMITER;
        }

        key if key.just_pressed(KeyCode::Down) => {
            let player_two = state.players.get_mut(1).unwrap();

            if player_two.direction <= -DIRECTION_LIMITER {
                return;
            }

            player_two.direction -= DIRECTION_LIMITER;
        }
        key if key.just_pressed(KeyCode::Up) => {
            let player_two = state.players.get_mut(1).unwrap();

            if player_two.direction >= DIRECTION_LIMITER {
                return;
            }

            player_two.direction += DIRECTION_LIMITER;
        }
        _ => (),
    }
}
