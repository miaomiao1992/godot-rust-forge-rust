use godot::{
    classes::{Area2D, AudioStream, IArea2D, TileMapLayer, node::ProcessMode},
    init::is_editor_hint,
    prelude::*,
};

use crate::{
    entities::audio_tirgger::AudioEffectType, message::Message, monster::enemy::Enemy,
    player::Player,
};

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub(crate) struct BossOrchestractor {
    base: Base<Area2D>,

    #[export]
    boss: OnEditor<Gd<Enemy>>,

    #[export]
    audio: OnEditor<Gd<AudioStream>>,

    #[export]
    door: OnEditor<Gd<TileMapLayer>>,
}

#[godot_api]
impl IArea2D for BossOrchestractor {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.base_mut().set_monitoring(true);
        self.boss.set_process_mode(ProcessMode::DISABLED);
        self.door.set_visible(false);
        self.door.set_enabled(false);

        self.base()
            .signals()
            .body_entered()
            .connect_other(&*self, Self::on_player_enter);
    }
}

impl BossOrchestractor {
    fn on_player_enter(&mut self, player: Gd<Node2D>) {
        if let Ok(_player) = player.try_cast::<Player>() {
            self.base_mut()
                .set_deferred("monitoring", &false.to_variant());
            let id = self.base().instance_id();
            let time = self
                .base()
                .get_tree()
                .create_timer(2.0)
                .signals()
                .timeout()
                .to_future();
            godot::task::spawn(async move {
                godot_print!("哈哈哈哈， 没想到你能来到这里。");
                time.await;

                if let Ok(mut node) = Gd::<BossOrchestractor>::try_from_instance_id(id) {
                    node.bind_mut().on_battle_start();
                }
            });
        }
    }

    fn on_battle_start(&mut self) {
        self.boss.set_process_mode(ProcessMode::INHERIT);
        self.door.set_visible(true);
        self.door.set_enabled(true);
        godot_print!("让你尝尝我的厉害");

        Message::singleton()
            .signals()
            .boss_battle_start()
            .emit(self.boss.get_name().to_string());
        Message::singleton()
            .signals()
            .play_music()
            .emit(&self.audio.clone(), AudioEffectType::LARGE);

        self.boss
            .signals()
            .tree_exiting()
            .connect_other(&*self, Self::on_battle_end);
        self.boss
            .signals()
            .health_change()
            .connect_other(&*self, Self::on_boss_health_change);
        let (hp, max_hp) = self.boss.bind().get_health_stats();
        self.on_boss_health_change(hp, max_hp);
    }

    fn on_battle_end(&mut self) {
        Message::singleton().signals().boss_battle_end().emit();
        self.door.set_enabled(false);
        self.door.set_visible(false);

        self.base_mut().queue_free();
    }

    fn on_boss_health_change(&mut self, hp: f32, max_hp: f32) {
        Message::singleton()
            .signals()
            .boss_health_change()
            .emit(hp, max_hp);
    }
}
