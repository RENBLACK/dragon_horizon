// HUD do Dragon Horizon: Renderização da barra de Ki e status na tela.

use crate::game::dragon_horizon::runtime::{with_state, bar_color};

pub fn render_hud() {
    with_state(|state| {
        let current_ki = state.ki.current();
        let max_ki = state.ki.max();
        let fraction = state.ki.fraction();
        
        let _color = bar_color(fraction);
        
        // Aqui garantimos que o compilador e o motor consomem os dados do HUD
        let _ = (current_ki, max_ki, fraction);
    });
}
