use godot::{classes::AudioStream, prelude::*};

use crate::{message::Message, monster::nega_boss::boss_state::IBossState};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossDash {
    base: Base<Node>,

    #[export]
    #[init(val = 300.0)]
    speed: f32,

    #[init(val = 0.0)]
    shadow_time: f32,

    #[export]
    #[init(val = 0.05)]
    shadow_duration: f32,

    #[export]
    #[init(val = 0.2)]
    duration: f32,

    #[init(val = 0.0)]
    time: f32,

    #[export]
    audio: OnEditor<Gd<AudioStream>>,
}

impl IBossState for BossDash {}

#[godot_api]
impl BossDash {
    #[func]
    fn entered(&mut self) {
        self.play_anim("dash");
        self.time = self.duration;
        self.shadow_time = self.shadow_duration;

        if let Some(player) = self.get_target() {
            let dir = self
                .get_agent()
                .get_global_position()
                .direction_to(player.get_global_position());
            self.get_agent().bind_mut().update_direction(dir);
            let velocity = self.speed * Vector2::RIGHT * dir;
            self.get_agent().set_velocity(velocity);
        }

        let pos = self.get_agent().get_global_position();

        Message::singleton()
            .signals()
            .play_spatial_audio()
            .emit(&self.audio.clone(), pos);
    }

    #[func]
    fn exited(&mut self) {}

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.time -= delta;

        if self.time <= 0.0 {
            // godot_print!("Dash 结束");
            return "Idle".to_variant();
        }

        self.shadow_time -= delta;

        if self.shadow_time <= 0.0 {
            self.shadow_time = self.shadow_duration;
            self.get_agent().bind_mut().make_shadow();
        }

        Variant::nil()
    }
}
