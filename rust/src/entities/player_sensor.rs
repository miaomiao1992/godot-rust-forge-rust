use godot::{
    classes::{Area2D, IArea2D},
    prelude::*,
};

use crate::monster::enemy::Enemy;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct PlayerSensor {
    base: Base<Area2D>,

    #[export]
    search_duration: f32,

    time: f32,
}

#[godot_api]
impl IArea2D for PlayerSensor {
    fn ready(&mut self) {
        //代码覆盖默认碰撞，检测玩家
        self.base_mut().set_collision_layer_value(1, false);
        self.base_mut().set_collision_mask_value(1, false);
        if let Some(node) = self.base().get_owner()
            && node.try_cast::<Enemy>().is_ok()
        {
            self.base_mut().set_collision_mask_value(3, true);
            self.signals()
                .body_entered()
                .connect_self(Self::on_player_enter);
            self.signals()
                .body_exited()
                .connect_self(Self::on_player_exit);
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.time > 0.0 {
            self.time -= delta as f32;

            if self.time <= 0.0 {
                self.signals().player_exited().emit();
            }
        }
    }
}

#[godot_api]
impl PlayerSensor {
    #[signal]
    pub fn player_entered(player: Gd<Node2D>);
    #[signal]
    pub fn player_exited();
    #[signal]
    pub fn started_searching();
}

impl PlayerSensor {
    fn on_player_enter(&mut self, player: Gd<Node2D>) {
        self.time = 0.0;
        self.signals().player_entered().emit(&player);
    }
    fn on_player_exit(&mut self, _player: Gd<Node2D>) {
        self.signals().started_searching().emit();
        self.time = self.search_duration;
    }
}
