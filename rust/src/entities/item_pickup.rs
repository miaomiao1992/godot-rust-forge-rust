use godot::{
    classes::{Area2D, AudioStream, IRigidBody2D, RigidBody2D},
    init::is_editor_hint,
    prelude::*,
};

use crate::message::Message;

#[derive(GodotClass)]
#[class(init, base = RigidBody2D)]
pub(crate) struct ItemPickUp {
    base: Base<RigidBody2D>,

    #[init(node = "%Area2D")]
    area2d: OnReady<Gd<Area2D>>,

    #[export]
    #[init(val = 1.0)]
    amount: f32,

    #[export]
    audio: OnEditor<Gd<AudioStream>>,
}

#[godot_api]
impl IRigidBody2D for ItemPickUp {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.area2d
            .signals()
            .body_entered()
            .connect_other(&*self, Self::on_player_enter);
    }
}

impl ItemPickUp {
    fn on_player_enter(&mut self, _node: Gd<Node2D>) {
        Message::singleton()
            .signals()
            .player_healed()
            .emit(self.amount);

        let pos = self.base().get_global_position();
        Message::singleton()
            .signals()
            .play_spatial_audio()
            .emit(&self.audio.clone(), pos);

        self.base_mut().call_deferred("queue_free", &[]);
    }
}
