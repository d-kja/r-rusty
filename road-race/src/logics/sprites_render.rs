use crate::*;

pub fn render_sprites(engine: &mut Engine, _: &mut GameState) {
    let window_width = engine.window_dimensions.x;

    let player = engine.add_sprite("player", SpritePreset::RacingCarBlack);

    player.translation.x = -(window_width / 2.0) + 100.0;
    player.scale = 0.75;
    player.layer = 10.0;
    player.collision = true;
}
