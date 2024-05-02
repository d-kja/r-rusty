use rand::prelude::*;
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    current_score: u32,
    ferris_index: isize,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            ferris_index: 0,
            spawn_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

impl GameState {
    fn increment_score(&mut self) {
        let score = self.current_score + 1;
        self.update_score(score);
    }

    fn reset_score(&mut self) {
        self.current_score = 0;
    }

    fn update_score(&mut self, value: u32) {
        self.current_score = value;
        self.update_high_score(value);
    }

    fn update_high_score(&mut self, value: u32) {
        if value < self.high_score {
            return;
        }

        self.high_score = value;
    }
}

const MOVEMENT_SPEED: f32 = 100.0;

fn main() {
    let mut game = Game::new();
    game.window_settings(Window {
        title: "Example".to_owned(),
        ..Default::default()
    });

    let player = game.add_sprite("player", SpritePreset::RacingCarBlack);

    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = 0.0;
    player.scale = 0.75;
    player.layer = 0.0; // z-index
    player.collision = true;

    // Creating score and high score texts
    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(-520.0, 320.0);

    let high_score = game.add_text("high_score", "High score: 0");
    high_score.translation = Vec2::new(520.0, 320.0);

    // Can have many and they're run based on the order it was inserted
    game.add_logic(render_score);
    game.add_logic(movement_logic);
    game.add_logic(other_controls);
    game.add_logic(game_layout);
    game.add_logic(collision_logic);
    game.add_logic(point_generation);

    // BGM
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.2);

    game.run(GameState::default());
}

fn movement_logic(engine: &mut Engine, _: &mut GameState) {
    let player = engine.sprites.get_mut("player").unwrap();

    match &engine.keyboard_state {
        key if key.pressed(KeyCode::ShiftLeft) => {
            let speed = MOVEMENT_SPEED * 2.0;

            match &engine.keyboard_state {
                key if key.pressed_any(&[KeyCode::W, KeyCode::Up]) => {
                    player.translation.y += speed * engine.delta_f32;
                    player.rotation = std::f32::consts::FRAC_PI_2;
                }
                key if key.pressed_any(&[KeyCode::S, KeyCode::Down]) => {
                    player.translation.y -= speed * engine.delta_f32;
                    player.rotation = std::f32::consts::PI * 3.0 / 2.0;
                }
                key if key.pressed_any(&[KeyCode::A, KeyCode::Left]) => {
                    player.translation.x -= speed * engine.delta_f32;
                    player.rotation = std::f32::consts::PI;
                }
                key if key.pressed_any(&[KeyCode::D, KeyCode::Right]) => {
                    player.translation.x += speed * engine.delta_f32;
                    player.rotation = 0.0;
                }
                _ => {}
            }
        }
        key if key.pressed_any(&[KeyCode::W, KeyCode::Up]) => {
            player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
            player.rotation = std::f32::consts::FRAC_PI_2;
        }
        key if key.pressed_any(&[KeyCode::S, KeyCode::Down]) => {
            player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
            player.rotation = std::f32::consts::PI * 3.0 / 2.0;
        }
        key if key.pressed_any(&[KeyCode::A, KeyCode::Left]) => {
            player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
            player.rotation = std::f32::consts::PI;
        }
        key if key.pressed_any(&[KeyCode::D, KeyCode::Right]) => {
            player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
            player.rotation = 0.0;
        }
        key if key.pressed(KeyCode::Delete) => engine.should_exit = true,
        _ => {}
    }
}

fn other_controls(engine: &mut Engine, state: &mut GameState) {
    match &engine.keyboard_state {
        key if key.pressed(KeyCode::R) => state.reset_score(),
        _ => {}
    }
}

fn game_layout(engine: &mut Engine, _: &mut GameState) {
    let window_width = engine.window_dimensions.x;
    let window_height = engine.window_dimensions.y;

    let time_offset = ((engine.time_since_startup_f64 * 2.0).cos() * 5.0) as f32;

    let score = engine.texts.get_mut("score").unwrap();

    score.translation.x = -window_width / 2.0 + 70.0;
    score.translation.y = window_height / 2.0 - 30.0 + time_offset;

    let high_score = engine.texts.get_mut("high_score").unwrap();

    high_score.translation.x = window_width / 2.0 - 100.0;
    high_score.translation.y = window_height / 2.0 - 30.0 + time_offset;
}

fn point_generation(engine: &mut Engine, state: &mut GameState) {
    if !state.spawn_timer.tick(engine.delta).just_finished() {
        return;
    }

    let label = format!("puppet-{}", state.ferris_index);
    let puppet = engine.add_sprite(label, SpritePreset::RollingBallBlue);

    puppet.translation.x = thread_rng().gen_range(-530.0..530.0);
    puppet.translation.y = thread_rng().gen_range(-320.0..320.0);
    puppet.scale = 0.75;
    puppet.collision = true;

    state.ferris_index += 1;
}

fn mouse_handling_example(engine: &mut Engine, state: &mut GameState) {
    if !engine.mouse_state.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(position) = engine.mouse_state.location() {
        let label = format!("puppet-{}", state.ferris_index);
        state.ferris_index += 1;

        let puppet = engine.add_sprite(label, SpritePreset::RollingBallBlue);
        puppet.translation = position;
        puppet.scale = 0.75;
        puppet.collision = true;
    }
}

fn collision_logic(engine: &mut Engine, state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for entity in event.pair.array() {
                if entity != "player" {
                    engine.sprites.remove(entity);
                }
            }

            state.increment_score();
            engine.audio_manager.play_sfx(SfxPreset::Click, 0.1);
        }
    }
}

fn render_score(engine: &mut Engine, state: &mut GameState) {
    let new_score = format!("Score: {:?}", state.current_score);
    let new_high_score = format!("High score: {:?}", state.high_score);

    let score = engine.texts.get_mut("score").unwrap();
    score.value = new_score;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.value = new_high_score;
}
