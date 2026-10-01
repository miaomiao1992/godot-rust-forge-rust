use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossAttack {
    base: Base<Node>,
    #[init(val = 0.0)]
    duration: f32,
}

impl IBossState for BossAttack {}

#[godot_api]
impl BossAttack {
    #[func]
    fn entered(&mut self) {
        self.play_anim("attack");
        self.get_agent().set_velocity(Vector2::ZERO);
        self.duration = self.get_anim_length("attack");
    }

    #[func]
    fn exited(&mut self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.duration -= delta;

        if self.duration <= 0.0 {
            return "Idle".to_variant();
        }

        Variant::nil()
    }
}
