#[allow(dead_code)]
// Lógica pura do mod Dragon Horizon: Ki e atributos.
// Não depende de nada do Horizon, então roda com `cargo test`.

use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Attributes {
    pub strength: u32,
    pub defense: u32,
    pub ki_power: u32,
    pub vitality: u32,
}

impl Default for Attributes {
    fn default() -> Self {
        Self { strength: 1, defense: 1, ki_power: 1, vitality: 1 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ki {
    current: f32,
    max: f32,
    regen_per_tick: f32,
}

impl Ki {
    pub fn new(max: f32, regen_per_tick: f32) -> Self {
        let max = max.max(1.0);
        Self { current: max, max, regen_per_tick: regen_per_tick.max(0.0) }
    }

    pub fn current(&self) -> f32 { self.current }
    pub fn max(&self) -> f32 { self.max }

    pub fn fraction(&self) -> f32 { self.current / self.max }

    pub fn spend(&mut self, amount: f32) -> bool {
        if amount < 0.0 || self.current < amount {
            return false;
        }
        self.current -= amount;
        true
    }

    pub fn regen(&mut self) {
        self.current = (self.current + self.regen_per_tick).min(self.max);
    }

    pub fn regen_ticks(&mut self, ticks: f32) {
        self.current = (self.current + self.regen_per_tick * ticks.max(0.0)).min(self.max);
    }

    pub fn set_max(&mut self, max: f32) {
        self.max = max.max(1.0);
        self.current = self.current.min(self.max);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Stat { Strength, Defense, KiPower, Vitality }

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub attrs: Attributes,
    pub ki: Ki,
    pub points: u32,
    last_update: Option<std::time::Instant>,
}

impl PlayerState {
    pub fn new() -> Self {
        let attrs = Attributes::default();
        Self { 
            ki: Ki::new(Self::ki_max_for(&attrs), 0.05), 
            attrs, 
            points: 0,
            last_update: None,
        }
    }

    fn ki_max_for(a: &Attributes) -> f32 {
        100.0 + a.ki_power as f32 * 20.0
    }

    pub fn add_points(&mut self, n: u32) { self.points += n; }

    pub fn upgrade(&mut self, which: Stat) -> bool {
        if self.points == 0 { return false; }
        self.points -= 1;
        match which {
            Stat::Strength => self.attrs.strength += 1,
            Stat::Defense => self.attrs.defense += 1,
            Stat::KiPower => self.attrs.ki_power += 1,
            Stat::Vitality => self.attrs.vitality += 1,
        }
        self.ki.set_max(Self::ki_max_for(&self.attrs));
        true
    }

    pub fn advance(&mut self, now: std::time::Instant) {
        if let Some(prev) = self.last_update {
            let secs = now.saturating_duration_since(prev).as_secs_f32().min(2.0);
            self.ki.regen_ticks(secs * 20.0);
        }
        self.last_update = Some(now);
    }
}

impl Default for PlayerState {
    fn default() -> Self { Self::new() }
}

static STATE: OnceLock<Mutex<PlayerState>> = OnceLock::new();

pub fn with_state<R>(f: impl FnOnce(&mut PlayerState) -> R) -> R {
    let m = STATE.get_or_init(|| Mutex::new(PlayerState::new()));
    let mut guard = m.lock().unwrap_or_else(|e| e.into_inner());
    f(&mut guard)
}

pub fn bar_color(fraction: f32) -> u32 {
    if fraction > 0.5 { 0xFF2E9BFF } else if fraction > 0.2 { 0xFFFFB02E } else { 0xFFFF3B3B }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spend_fails_without_enough_ki() {
        let mut ki = Ki::new(50.0, 1.0);
        assert!(!ki.spend(60.0));
        assert_eq!(ki.current(), 50.0);
    }

    #[test]
    fn regen_is_capped_at_max() {
        let mut ki = Ki::new(10.0, 5.0);
        ki.spend(2.0);
        ki.regen();
        ki.regen();
        assert_eq!(ki.current(), 10.0);
    }

    #[test]
    fn upgrade_needs_points_and_raises_max_ki() {
        let mut p = PlayerState::new();
        let before = p.ki.max();
        assert!(!p.upgrade(Stat::KiPower));
        p.add_points(1);
        assert!(p.upgrade(Stat::KiPower));
        assert!(p.ki.max() > before);
    }

    #[test]
    fn fraction_is_between_zero_and_one() {
        let mut ki = Ki::new(100.0, 0.0);
        ki.spend(25.0);
        assert!((ki.fraction() - 0.75).abs() < 1e-6);
    }

    #[test]
    fn advance_regens_by_elapsed_time() {
        use std::time::{Duration, Instant};
        let mut p = PlayerState::new();
        p.ki.spend(50.0);
        let t0 = Instant::now();
        p.advance(t0);
        let before = p.ki.current();
        p.advance(t0 + Duration::from_secs(1));
        assert!(p.ki.current() > before);
    }
}
