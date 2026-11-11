fn dragon_horizon_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    let (fraction, current, max) = crate::game::dragon_horizon::with_state(|s| {
        s.advance(std::time::Instant::now());
        (s.ki.fraction(), s.ki.current(), s.ki.max())
    });

    let (w, h) = (80.0_f32, 8.0_f32);
    let (x, y) = corners.place(horizon_client::extras::Corner::BottomLeft, w, h + 10.0);

    game.native_fill(batch, [x, y + 10.0, w, h], 0xFF202020);
    game.native_fill(batch, [x, y + 10.0, w * fraction, h], crate::game::dragon_horizon::bar_color(fraction));
    game.native_text(batch, &format!("Ki {:.0}/{:.0}", current, max), [x, y], 0xFFFFFF, true);
}
