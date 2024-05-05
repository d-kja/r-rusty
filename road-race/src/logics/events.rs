use crate::*;

pub fn handle_collision(engine: &mut Engine, state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        if event.state.eq(&CollisionState::End) {
            continue;
        }

        if event.pair.one_starts_with("obstacle-") && event.pair.one_starts_with("player") {
            state.health -= 1;

            // losing control
            let new_direction = match state.direction {
                x if x.is_sign_positive() => -1.0,
                x if x.is_sign_negative() => 1.0,
                _ => thread_rng().gen_range(-1..1) as f32,
            };

            state.direction = new_direction; // 😉
        }
    }
}
