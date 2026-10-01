use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossIdle {
    base: Base<Node>,
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
            return "Walk".to_variant();
        }
        Variant::nil()
    }
}
