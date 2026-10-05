use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossDeath {
    base: Base<Node>,

    #[init(val = 0.0)]
    duration: f32,
}

impl IBossState for BossDeath {}

#[godot_api]
impl BossDeath {
    #[func]
    fn entered(&mut self) {
        self.get_agent().bind_mut().set_damage_disable(true);
        self.play_anim("death");
        self.duration = self.get_anim_length("death");
        self.get_agent().set_velocity(Vector2::ZERO);
    }

    #[func]
    fn exited(&mut self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.duration -= delta;

        if self.duration <= 0.0 {
            let pos = self.get_agent().get_global_position();
            self.get_agent().signals().death().emit(pos);
            self.get_agent().call_deferred("queue_free", &[]);
        }
        Variant::nil()
    }
}
