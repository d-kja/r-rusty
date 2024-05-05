use std::{thread, time::Duration};

use crate::*;

use super::movement::is_dead;

pub fn render_layout(engine: &mut Engine, state: &mut GameState) {
    let window = engine.window_dimensions;
    let height = window.y / 2.0;
    let width = window.x / 2.0;

    let health_text = engine.add_text("health", format!("{} HP", state.health));
    health_text.translation = Vec2::new((width * -1.0) + 60.0, (height) - 40.0);

    if is_dead(state.health) {
        let gameover_text = engine.add_text("gameover", "DEADGE");
        gameover_text.font_size = 127.0;

        engine.audio_manager.stop_music();
    }
}
