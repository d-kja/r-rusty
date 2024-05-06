use super::movement::are_both_dead;
use crate::*;

pub fn render_layout(engine: &mut Engine, state: &mut GameState) {
    let window = engine.window_dimensions;
    let height = window.y / 2.0;
    let width = window.x / 2.0;

    let health_one = engine.add_text(
        "health-0",
        format!(
            "Player one's HP: {}",
            state
                .players
                .get(0)
                .expect("unable to get player one's health")
                .health
        ),
    );
    health_one.translation = Vec2::new((width * -1.0) + 120.0, (height) - 40.0);

    let health_two = engine.add_text(
        "health-1",
        format!(
            "Player two's HP: {}",
            state
                .players
                .get(1)
                .expect("unable to get player two's health")
                .health
        ),
    );
    health_two.translation = Vec2::new((width * -1.0) + 120.0, (height) - 80.0);

    if are_both_dead(&state.players) {
        let gameover_text = engine.add_text("gameover", "DEADGE");
        gameover_text.font_size = 127.0;

        engine.audio_manager.stop_music();
    }
}
