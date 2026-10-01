use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossWalk {
    base: Base<Node>,
    #[export]
    #[init(val = 50.0)]
    speed: f32,
}

impl IBossState for BossWalk {}

#[godot_api]
impl BossWalk {
    #[func]
    fn entered(&mut self) {
        self.play_anim("walk");
    }

    #[func]
    fn exited(&mut self) {}

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        if let Some(player) = self.get_target() {
            let mut agent = self.get_agent();
            let dir = agent
                .get_global_position()
                .direction_to(player.get_global_position());
            agent.bind_mut().update_direction(dir);
            if agent.bind().is_in_attack_range() {
                return "Attack".to_variant();
            }
            agent.set_velocity(self.speed * dir);
            return Variant::nil();
        } else {
            return "Idle".to_variant();
        }
    }
}
