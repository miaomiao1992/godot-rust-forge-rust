use godot::{
    classes::{ISprite2D, Sprite2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub(crate) struct EnegryWave {
    base: Base<Sprite2D>,
    #[export]
    #[init(val = 200.0)]
    speed: f32,

    #[export]
    #[init(val = 5.0)]
    duration: f32,

    #[init(val = true)]
    face_to_right: bool,
}

impl EnegryWave {
    pub fn set_face_to_right(&mut self, v: bool) {
        self.face_to_right = v;
    }
}

#[godot_api]
impl ISprite2D for EnegryWave {
    fn process(&mut self, delta: f64) {
        let mut pos = self.base().get_global_position();

        let delta = delta as f32;

        pos.x += self.speed * delta * (if self.face_to_right { 1.0 } else { -1.0 });

        self.base_mut().set_global_position(pos);

        self.duration -= delta;

        if self.duration <= 0.0 {
            self.base_mut().call_deferred("queue_free", &[]);
        }
    }
}
