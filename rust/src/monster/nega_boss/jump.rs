use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossJump {
    base: Base<Node>,
    #[export]
    #[init(val = 60.0)]
    control_height: f32,

    #[export(range = (0.0, 1.0, 0.01))]
    #[init(val = 0.8)]
    center_rate: f32,
}

impl IBossState for BossJump {}

#[godot_api]
impl BossJump {
    #[func]
    fn entered(&mut self) {
        self.play_anim("jump");
        self.clac_velocity();
    }

    #[func]
    fn exited(&mut self) {}

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        if self.get_agent().is_on_floor() {
            return "Idle".to_variant();
        }
        Variant::nil()
    }

    fn clac_velocity(&self) {
        let agent_pos = self.get_agent().get_global_position();
        if let Some(player) = self.get_target() {
            let player_pos = player.get_global_position();

            let center_x = agent_pos.x + (player_pos.x - agent_pos.x) * self.center_rate;
            let center_y = player_pos.y - self.control_height;
            // let center = Vector2::new(center_x, center_y);
            let gravity = player.get_gravity().y;
            //0.5 * a * t * t = h
            let jump_height = center_y - agent_pos.y;
            let duration = (2.0 * jump_height / gravity).abs().sqrt();
            let vel_y = -duration * gravity;
            let vel_x = (center_x - agent_pos.x) / duration;
            let velocity = Vector2::new(vel_x, vel_y);
            self.get_agent().set_velocity(velocity);
        }
    }
}
