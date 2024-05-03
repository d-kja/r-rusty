use crate::*;

pub const SPEED: f32 = 250.0;
const Y_BOUNDARY: f32 = 360.0;

pub fn player_movement(engine: &mut Engine, state: &mut GameState) {
    let direction = state.direction;
    let player = engine
        .sprites
        .get_mut("player")
        .expect("unable to find player sprite");

    match player.translation.y {
        pos if (pos > Y_BOUNDARY && direction.is_sign_positive()
            || pos < -Y_BOUNDARY && direction.is_sign_negative()) =>
        {
            if is_dead(state.health) {
                return;
            }

            state.health -= 1;
        }
        _ => {
            if is_dead(state.health) {
                return;
            }

            player.rotation = direction * 0.2;
            player.translation.y += direction * SPEED * engine.delta_f32;
        }
    }
}

fn is_dead(health: u8) -> bool {
    health.eq(&0)
}
