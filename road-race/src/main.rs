use rand::prelude::*;
use rusty_engine::prelude::bevy::utils::petgraph::matrix_graph::Zero;
pub use rusty_engine::prelude::*;

mod logics;
use logics::{
    events::handle_collision,
    inputs::control_inputs,
    movement::{obstacle_movement, player_one_movement, player_two_movement, road_movement},
    render::render_layout,
};

#[derive(Clone)]
pub struct Player {
    health: u8,
    direction: f32,
    current_rotation: f32,
    preset: SpritePreset,
    label: String,
}

impl Player {
    pub fn new(label: &str, preset: SpritePreset) -> Self {
        Self {
            health: 25,
            label: label.to_string(),
            preset,
            direction: 0.0,
            current_rotation: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct GameState {
    players: [Player; 2],
    obstacles: Vec<SpritePreset>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            players: [
                Player::new("player-1", SpritePreset::RacingCarBlack),
                Player::new("player-2", SpritePreset::RacingCarRed),
            ],
            obstacles: vec![
                SpritePreset::RacingBarrierWhite,
                SpritePreset::RacingConeStraight,
                SpritePreset::RacingBarrierRed,
            ],
        }
    }
}

fn main() {
    let mut game = Game::new();
    let game_state = GameState::default();

    let dimensions = Vec2::new(1280.0, 720.0);

    game.window_settings(Window {
        title: "Road race".to_owned(),
        window_theme: Some(bevy::window::WindowTheme::Dark),
        resizable: false,
        resolution: WindowResolution::new(dimensions.x, dimensions.y),
        ..Default::default()
    });

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.15);

    // Render road lines
    let window_width = if !game.window_dimensions.x.is_zero() {
        game.window_dimensions.x
    } else {
        dimensions.x
    };
    let window_height = if !game.window_dimensions.y.is_zero() {
        game.window_dimensions.y
    } else {
        dimensions.y
    };

    let window_half_half_screen = window_height / 2.0 / 2.0;

    // Render players sprite once.
    for (idx, player) in game_state.players.clone().into_iter().enumerate() {
        let player = game.add_sprite(&player.label, player.preset);

        player.scale = 0.75;
        player.layer = 10.0;
        player.collision = true;

        player.translation.y = if idx == 0 {
            -window_half_half_screen
        } else {
            window_half_half_screen
        };
    }

    let roadline_gap = 100.0;
    let initial_roadline_amount = (window_width / roadline_gap) as u32;
    let roadline_range = if initial_roadline_amount % 2 == 0 {
        0..(initial_roadline_amount + 1) // inclusive range has it's own type 🥴
    } else {
        0..initial_roadline_amount
    };

    // Lines in between
    for idx in 0..(window_width / 10.0) as u32 {
        let sprite = game.add_sprite(format!("divider-{}", idx), SpritePreset::RacingBarrierWhite);

        sprite.scale = 0.1;

        sprite.translation.x = idx as f32 * 19.0 - window_width;
    }

    // First road
    for idx in roadline_range.clone() {
        let road_sprite = game.add_sprite(
            format!("roadline-first-{}", idx),
            SpritePreset::RacingBarrierWhite,
        );

        road_sprite.scale = 0.1;

        // translating to - window_width / 2.0 is bugging for some reason...
        road_sprite.translation.x = roadline_gap * idx as f32;
        road_sprite.translation.y = -window_half_half_screen;
    }

    // Second road
    for idx in roadline_range {
        let road_sprite = game.add_sprite(
            format!("roadline-second-{}", idx),
            SpritePreset::RacingBarrierWhite,
        );

        road_sprite.scale = 0.1;

        // translating to - window_width / 2.0 is bugging for some reason...
        road_sprite.translation.x = roadline_gap * idx as f32;
        road_sprite.translation.y = window_half_half_screen;
    }

    // Render obstacles
    for (idx, &preset) in game_state.obstacles.iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle-{}", idx), preset);

        obstacle.layer = 5.0;
        obstacle.scale = 0.75;
        obstacle.collision = true;

        let width = window_width as i32;
        let height = window_height as i32;

        let pos = generate_obstacle_pos(width, height);

        obstacle.translation = Vec2::new(pos.0, pos.1);
    }

    // Logics
    game.add_logic(control_inputs);
    game.add_logic(render_layout);
    game.add_logic(player_one_movement);
    game.add_logic(player_two_movement);
    game.add_logic(road_movement);
    game.add_logic(obstacle_movement);
    game.add_logic(handle_collision);

    game.run(game_state);
}

pub fn generate_obstacle_pos(width: i32, height: i32) -> (f32, f32) {
    let pos_x = thread_rng().gen_range((width / 2)..(width + 300)) as f32;
    let pos_y = thread_rng().gen_range(-(height / 2)..(height / 2)) as f32;

    (pos_x, pos_y)
}
