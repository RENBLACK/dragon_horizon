fn render_dragon_hud() {
    crate::game::dragon_horizon::runtime::with_state(|state| {
        let current_ki = state.ki.current();
        let max_ki = state.ki.max();
        let fraction = state.ki.fraction();
        
        let _color = crate::game::dragon_horizon::runtime::bar_color(fraction);
        let _ = (current_ki, max_ki, fraction);
    });
}
