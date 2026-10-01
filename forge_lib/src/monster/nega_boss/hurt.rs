use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossHurt {
    base: Base<Node>,
    #[export]
    #[init(val = 50.0)]
    speed: f32,

    #[init(val = 0.0)]
    duration: f32,
}

impl IBossState for BossHurt {}

#[godot_api]
impl BossHurt {
    #[func]
    fn entered(&mut self) {
        self.get_agent().bind_mut().set_damage_disable(true);
        self.play_anim("hurt");
        self.duration = self.get_anim_length("hurt");
        if let Some(player) = self.get_target() {
            let mut agent = self.get_agent();
            let dir = agent
                .get_global_position()
                .direction_to(player.get_global_position());
            agent.bind_mut().update_direction(dir);
            agent.set_velocity(self.speed * dir * -1.0);
        }
    }

    #[func]
    fn exited(&mut self) {
        self.get_agent().bind_mut().set_damage_disable(false);
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.duration -= delta;
        if self.duration <= 0.0 {
            let (hp, ..) = self.get_agent().bind().get_health_stats();
            if hp <= 0.0 {
                return "Death".to_variant();
            }

            return "Idle".to_variant();
        }

        Variant::nil()
    }
}
