use crate::*;

use super::movement::is_dead;

pub fn handle_collision(engine: &mut Engine, state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        if event.state.eq(&CollisionState::End) {
            continue;
        }

        if event.pair.one_starts_with("obstacle-") && event.pair.one_starts_with("player-") {
            let player_label = event
                .pair
                .into_iter()
                .find(|item| item.starts_with("player-"))
                .unwrap();

            match &player_label[..] {
                "player-1" => {
                    let player = state.players.get_mut(0).unwrap();

                    if is_dead(player) {
                        return;
                    }

                    player.health -= 1;

                    // losing control
                    let new_direction = match player.direction {
                        x if x.is_sign_positive() => -1.0,
                        x if x.is_sign_negative() => 1.0,
                        _ => thread_rng().gen_range(-1..1) as f32,
                    };

                    player.direction = new_direction; // 😉
                }
                "player-2" => {
                    let player = state.players.get_mut(1).unwrap();

                    if is_dead(player) {
                        return;
                    }

                    player.health -= 1;

                    // losing control
                    let new_direction = match player.direction {
                        x if x.is_sign_positive() => -1.0,
                        x if x.is_sign_negative() => 1.0,
                        _ => thread_rng().gen_range(-1..1) as f32,
                    };

                    player.direction = new_direction; // 😉
                }
                _ => (),
            }
        }
    }
}
