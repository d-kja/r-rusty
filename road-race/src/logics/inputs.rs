use crate::*;

pub fn control_inputs(engine: &mut Engine, state: &mut GameState) {
    let key_events = &engine.keyboard_state;

    match key_events {
        key if key.just_pressed_any(&[KeyCode::S, KeyCode::Down]) => {
            if state.direction < 0.0 {
                return;
            }

            state.direction -= 1.0;
        }
        key if key.just_pressed_any(&[KeyCode::W, KeyCode::Up]) => {
            if state.direction > 0.0 {
                return;
            }

            state.direction += 1.0;
        }
        _ => (),
    }
}
