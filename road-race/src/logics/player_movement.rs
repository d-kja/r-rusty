use crate::*;

pub const SPEED: f32 = 250.0;

pub fn player_movement(engine: &mut Engine, state: &mut GameState) {
    let direction = state.direction;
    let player = engine
        .sprites
        .get_mut("player")
        .expect("unable to find player sprite");

    println!(
        "{}, direction: {direction}",
        direction * SPEED * engine.delta_f32
    );

    player.translation.y += direction * SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;
}
