use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossIdle {
    base: Base<Node>,

    #[export]
    #[init(val = 100.0)]
    dash_emit_distance: f32,
}

impl IBossState for BossIdle {}

#[godot_api]
impl BossIdle {
    #[func]
    fn entered(&mut self) {
        self.play_anim("idle");
        self.get_agent().set_velocity(Vector2::ZERO);
    }

    #[func]
    fn exited(&mut self) {}

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        if self.get_target().is_some() {
            if self.get_agent().bind().is_in_attack_range() {
                return "Attack".to_variant();
            }
            if self.get_agent().bind().is_player_on_above_floor() {
                return "Jump".to_variant();
            }
            if self.get_agent().bind().is_player_on_below_floor() {
                return "Fall".to_variant();
            }

            if let Some(player) = self.get_target() {
                let pos = self.get_agent().get_global_position();
                let p_pos = player.get_global_position();
                let distance_x = (pos.x - p_pos.x).abs();
                if !self.get_agent().bind().is_in_attack_range()
                    && distance_x > self.dash_emit_distance
                    && self.get_agent().bind().not_in_dash_cooldown()
                {
                    return "Dash".to_variant();
                }
            }

            return "Walk".to_variant();
        }
        Variant::nil()
    }
}
