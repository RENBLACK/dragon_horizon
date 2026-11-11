// HUD do Dragon Horizon: Renderização da barra de Ki e status na tela.

use crate::game::dragon_horizon::runtime::{with_state, bar_color};

// A regra do Horizon exige exatamente uma única função privada aqui
fn render_dragon_hud() {
    with_state(|state| {
        let current_ki = state.ki.current();
        let max_ki = state.ki.max();
        let fraction = state.ki.fraction();
        
        let _color = bar_color(fraction);
        let _ = (current_ki, max_ki, fraction);
    });
}
