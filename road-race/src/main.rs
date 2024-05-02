pub use rusty_engine::prelude::*;

mod logics;
use logics::{
    inputs::control_inputs, player_movement::player_movement, sprites_render::render_sprites,
};

#[derive(Resource)]
pub struct GameState {
    health: u8,
    deadge: bool,
    direction: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health: 10,
            deadge: false,
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

    // Logics
    game.add_logic(render_sprites);
    game.add_logic(control_inputs);
    game.add_logic(player_movement);

    game.run(game_state);
}
