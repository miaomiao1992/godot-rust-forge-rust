use godot::{classes::AudioStream, prelude::*};

use crate::{message::Message, monster::nega_boss::boss_state::IBossState};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossFall {
    base: Base<Node>,
    #[init(val = 0.0)]
    duration: f32,
    #[export]
    audio: OnEditor<Gd<AudioStream>>,
}

impl IBossState for BossFall {}

#[godot_api]
impl BossFall {
    #[func]
    fn entered(&mut self) {
        self.play_anim("fall");
        self.duration = self.get_anim_length("fall");
        self.get_agent().bind_mut().set_collider_disable(true);
    }

    #[func]
    fn exited(&mut self) {
        self.get_agent().bind_mut().set_collider_disable(false);
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.duration -= delta;
        if self.duration <= 0.0 {
            self.get_agent().bind_mut().set_collider_disable(false);
            if self.get_agent().is_on_floor() {
                let pos = self.get_agent().get_global_position();
                Message::singleton()
                    .signals()
                    .play_spatial_audio()
                    .emit(&self.audio.clone(), pos);
                return "Idle".to_variant();
            }
        }
        let mut velocity = self.get_agent().get_velocity();
        velocity += self.get_agent().get_gravity() * delta;
        velocity.x = 0.0;
        self.get_agent().set_velocity(velocity);

        Variant::nil()
    }
}
