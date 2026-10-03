use godot::{classes::AudioStream, prelude::*};

use crate::{
    managers::visual_effect::VisualEffectType, message::Message,
    monster::nega_boss::boss_state::IBossState,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossSlam {
    base: Base<Node>,
    #[export]
    audio: OnEditor<Gd<AudioStream>>,
}

impl IBossState for BossSlam {}

#[godot_api]
impl BossSlam {
    #[func]
    fn entered(&mut self) {
        self.get_agent().bind_mut().set_damage_disable(true);
        self.play_anim("slam");
        self.get_agent().bind_mut().set_gravity_scale(5.0);
    }

    #[func]
    fn exited(&mut self) {
        self.get_agent().bind_mut().set_fly_slam();
        self.get_agent().bind_mut().set_damage_disable(false);
        self.get_agent().bind_mut().set_gravity_scale(1.0);
        self.get_agent().bind_mut().create_enegry_wave();
    }

    #[func]
    fn update(&mut self, _delta: f64) -> Variant {
        if self.get_agent().is_on_floor() {
            let pos = self.get_agent().get_global_position();
            Message::singleton()
                .signals()
                .play_spatial_audio()
                .emit(&self.audio.clone(), pos);
            Message::singleton()
                .signals()
                .play_effect()
                .emit(VisualEffectType::Land, pos);
            Message::singleton().signals().camera_shake().emit(20.0);
            return "Recover".to_variant();
        }
        Variant::nil()
    }
}
