use godot::{
    classes::{Area2D, ISprite2D, Sprite2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct Bullet {
    base: Base<Sprite2D>,

    #[init(val = 300.0)]
    speed: f32,

    #[init(val = Vector2::RIGHT)]
    dir_axios: Vector2,

    #[init(node = "%Area2D")]
    area: OnReady<Gd<Area2D>>,

    #[init(val = 5.0)]
    duration: f32,
}

#[godot_api]
impl ISprite2D for Bullet {
    fn ready(&mut self) {
        self.area
            .signals()
            .body_entered()
            .connect_other(&*self, Self::on_bullet_collide);
    }

    fn process(&mut self, delta: f64) {
        let delta = delta as f32;
        let mut pos = self.base().get_global_position();
        pos.x += self.dir_axios.x * self.speed * delta;
        self.base_mut().set_global_position(pos);

        self.duration -= delta;

        if self.duration <= 0.0 {
            self.base_mut().call_deferred("queue_free", &[]);
        }
    }
}

impl Bullet {
    fn on_bullet_collide(&mut self, _b: Gd<Node2D>) {
        self.base_mut().call_deferred("queue_free", &[]);
    }

    pub fn set_dir(&mut self, v: Vector2) {
        self.dir_axios = v;
        if v.x > 0.0 {
            self.base_mut().set_scale(Vector2::new(-1.0, 1.0));
        } else {
            self.base_mut().set_scale(Vector2::new(1.0, 1.0));
        }
    }
}
