use crate::*;

const ROAD_SPEED: f32 = 400.0;
const OBSTACLE_SPEED: f32 = 400.0;
pub const PLAYER_SPEED: f32 = 250.0;
const Y_BOUNDARY: f32 = 360.0;

pub fn player_movement(engine: &mut Engine, state: &mut GameState) {
    if is_dead(state.health) {
        return;
    }

    let direction = state.direction;
    let player = engine
        .sprites
        .get_mut("player")
        .expect("unable to find player sprite");

    let game_width = -engine.window_dimensions.x;

    match player.translation.y {
        pos if (pos > Y_BOUNDARY && direction.is_sign_positive()
            || pos < -Y_BOUNDARY && direction.is_sign_negative()) =>
        {
            state.health -= 1; // I mean... gotta give it some leeway
        }
        _ => {
            if is_dead(state.health) {
                return;
            }

            player.rotation = direction * 0.2;
            player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
            player.translation.x = game_width / 2.0 + 120.0;
        }
    }
}

pub fn road_movement(engine: &mut Engine, state: &mut GameState) {
    if is_dead(state.health) {
        return;
    }

    let sprites = engine.sprites.values_mut();

    for sprite in sprites {
        if !sprite.label.contains("roadline-") {
            continue;
        }

        sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

        // could replace with amount of roads + gap of each to get an idea how many "pixels" I'd
        // have to travel to move them to the right most position
        let window_width = engine.window_dimensions.x;

        if sprite.translation.x <= -(window_width / 1.5) {
            sprite.translation.x = window_width + 250.0;
        }
    }
}

pub fn obstacle_movement(engine: &mut Engine, state: &mut GameState) {
    if is_dead(state.health) {
        return;
    }

    let sprites = engine.sprites.values_mut();

    for sprite in sprites {
        if !sprite.label.starts_with("obstacle-") {
            continue;
        }

        sprite.translation.x -= OBSTACLE_SPEED * engine.delta_f32;

        let window_width = engine.window_dimensions.x;
        let window_height = engine.window_dimensions.y;

        if sprite.translation.x <= -(window_width / 2.0) {
            let (x, y) = generate_obstacle_pos(window_width as i32, window_height as i32);

            sprite.translation = Vec2::new(x, y);
        }
    }
}

pub fn is_dead(health: u8) -> bool {
    health.is_zero()
}
