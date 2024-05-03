pub use rusty_engine::prelude::*;

mod logics;
use logics::{inputs::control_inputs, movement::player_movement, render::render_layout};

#[derive(Resource)]
pub struct GameState {
    health: u8,
    direction: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health: 10,
            direction: 0.0,
        }
    }
}

fn main() {
    let mut game = Game::new();
    let game_state = GameState::default();

    game.window_settings(Window {
        title: "Road race".to_owned(),
        ..Default::default()
    });

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.15);

    // Render player sprite once.
    let player = game.add_sprite("player", SpritePreset::RacingCarBlack);

    player.translation.x = -550.0;
    player.scale = 0.75;
    player.layer = 10.0;
    player.collision = true;

    // Logics
    game.add_logic(control_inputs);
    game.add_logic(render_layout);
    game.add_logic(player_movement);

    game.run(game_state);
}
