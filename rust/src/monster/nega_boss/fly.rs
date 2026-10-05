use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossFly {
    base: Base<Node>,

    #[export]
    #[init(val = 100.0)]
    speed: f32,

    #[export]
    #[init(val = Vector2::new(-390.0, 440.0))]
    air_marker: Vector2,

    #[export]
    #[init(val = 2.0)]
    duration: f32,

    #[export]
    #[init(val = 0.5)]
    animation_duration: f32,

    #[init(val = 0.0)]
    time: f32,
}

impl IBossState for BossFly {}

#[godot_api]
impl BossFly {
    #[func]
    fn entered(&mut self) {
        self.get_agent().set_velocity(Vector2::ZERO);
        self.get_agent().bind_mut().set_damage_disable(true);
        self.play_anim("fly");
        self.get_agent().bind_mut().set_gravity_scale(0.0);

        let mut tween = self.base_mut().create_tween();
        let target_pos = self.air_marker;
        tween.tween_property(
            &self.get_agent(),
            "global_position",
            &target_pos.to_variant(),
            self.animation_duration as f64,
        );
        self.time = self.duration;
    }

    #[func]
    fn exited(&mut self) {
        self.get_agent().bind_mut().set_damage_disable(false);
        self.get_agent().bind_mut().set_gravity_scale(1.0);
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.time -= delta;

        if self.time <= 0.0 {
            return "Slam".to_variant();
        }
        Variant::nil()
    }
}
