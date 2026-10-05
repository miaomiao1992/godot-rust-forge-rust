use godot::{classes::AudioStream, prelude::*};

use crate::{message::Message, monster::nega_boss::boss_state::IBossState};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossRecover {
    base: Base<Node>,

    #[export]
    #[init(val = 3.0)]
    duration: f32,

    #[init(val = 0.0)]
    recover_time: f32,

    #[export]
    #[init(val = 1.0)]
    recover_gap: f32,

    #[init(val = 0.0)]
    time: f32,

    #[export]
    audio: OnEditor<Gd<AudioStream>>,
}

impl IBossState for BossRecover {}

#[godot_api]
impl BossRecover {
    #[func]
    fn entered(&mut self) {
        self.get_agent().bind_mut().set_damage_disable(true);
        self.play_anim("recover");
        self.get_agent().set_velocity(Vector2::ZERO);
        self.time = 0.0;
        self.recover_time = self.duration;
        self.get_agent().bind_mut().set_attack_disable(true);
    }

    #[func]
    fn exited(&mut self) {
        self.get_agent()
            .call_deferred("set_attack_disable", &[false.to_variant()]);
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.recover_time -= delta;
        self.time += delta;

        if self.time >= self.recover_gap {
            self.time = 0.0;
            self.get_agent().bind_mut().heal_health(1.0);
            let pos = self.get_agent().get_global_position();
            Message::singleton()
                .signals()
                .play_spatial_audio()
                .emit(&self.audio.clone(), pos);
        }

        if self.recover_time <= 0.0 {
            return "Idle".to_variant();
        }

        Variant::nil()
    }
}
