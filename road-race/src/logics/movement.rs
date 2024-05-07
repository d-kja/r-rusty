use crate::*;

const ROAD_SPEED: f32 = 400.0;
const OBSTACLE_SPEED: f32 = 400.0;
pub const PLAYER_SPEED: f32 = 250.0;
const Y_BOUNDARY: f32 = 360.0;

pub fn player_one_movement(engine: &mut Engine, state: &mut GameState) {
    let player = state.players.get_mut(0).unwrap();

    if is_dead(player) {
        return;
    }

    let direction = player.direction;
    let player_one = engine
        .sprites
        .get_mut("player-1")
        .expect("unable to find player_one sprite");

    let game_width = -engine.window_dimensions.x;

    match player_one.translation.y {
        pos if (pos > Y_BOUNDARY && direction.is_sign_positive()
            || pos < -Y_BOUNDARY && direction.is_sign_negative()) =>
        {
            player.health -= 1; // I mean... gotta give it some leeway
        }
        _ => {
            if is_dead(player) {
                return;
            }

            player_one.rotation = direction * 0.2;
            player_one.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
            player_one.translation.x = game_width / 2.0 + 120.0;
        }
    }
}

pub fn player_two_movement(engine: &mut Engine, state: &mut GameState) {
    let player = state.players.get_mut(1).unwrap();

    if is_dead(player) {
        return;
    }

    let direction = player.direction;
    let player_two = engine
        .sprites
        .get_mut("player-2")
        .expect("unable to find player_two sprite");

    let game_width = -engine.window_dimensions.x;

    match player_two.translation.y {
        pos if (pos > Y_BOUNDARY && direction.is_sign_positive()
            || pos < -Y_BOUNDARY && direction.is_sign_negative()) =>
        {
            player.health -= 1; // I mean... gotta give it some leeway
        }
        _ => {
            if is_dead(player) {
                return;
            }

            player_two.rotation = direction * 0.2;
            player_two.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
            player_two.translation.x = game_width / 2.0 + 120.0;
        }
    }
}

pub fn road_movement(engine: &mut Engine, state: &mut GameState) {
    if are_both_dead(&state.players) {
        return;
    }

    let sprites = engine.sprites.values_mut();

    for sprite in sprites {
        if !sprite.label.contains("roadline") {
            continue;
        }

        sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

        // could replace with amount of roads + gap of each to get an idea how many "pixels" I'd
        // have to travel to move them to the right most position
        let window_width = engine.window_dimensions.x;

        if sprite.translation.x <= -(window_width / 2.0) {
            sprite.translation.x = window_width / 2.0;
        }
    }
}

pub fn obstacle_movement(engine: &mut Engine, state: &mut GameState) {
    if are_both_dead(&state.players) {
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

pub fn are_both_dead(players: &[Player; 2]) -> bool {
    players[0].health.is_zero() && players[1].health.is_zero()
}

pub fn is_dead(player: &Player) -> bool {
    player.health.is_zero()
}
