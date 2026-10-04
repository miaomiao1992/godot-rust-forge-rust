use godot::{
    classes::{
        AnimationPlayer, Button, CanvasLayer, ICanvasLayer, Label, MarginContainer, ProgressBar,
        TextureProgressBar, Tween,
        tween::{EaseType, TransitionType},
    },
    prelude::*,
};

use crate::{message::Message, resource::particles::HitParticleSetting};

#[derive(GodotClass)]
#[class(init, base = CanvasLayer)]
struct PlayerHud {
    base: Base<CanvasLayer>,
    #[init(node = "%HPMarginContainer")]
    hp_container: OnReady<Gd<MarginContainer>>,
    #[init(node = "%HPBar")]
    hp_bar: OnReady<Gd<TextureProgressBar>>,
    #[init(node = "%Button")]
    button: OnReady<Gd<Button>>,
    #[export]
    particle_setting: OnEditor<Gd<HitParticleSetting>>,

    #[init(node = "%Label")]
    boss_name: OnReady<Gd<Label>>,
    #[init(node = "%ProgressBar")]
    boss_hp_bar: OnReady<Gd<ProgressBar>>,
    #[init(node = "%HpHightlight")]
    boss_hp_hightlight: OnReady<Gd<ProgressBar>>,
    #[init(node = "%AnimationPlayer")]
    boss_anim: OnReady<Gd<AnimationPlayer>>,

    #[init(val = None)]
    tween: Option<Gd<Tween>>,
}

#[godot_api]
impl ICanvasLayer for PlayerHud {
    fn ready(&mut self) {
        Message::singleton()
            .signals()
            .player_health_change()
            .connect_other(&*self, Self::on_health_change);
        Message::singleton()
            .signals()
            .boss_battle_start()
            .connect_other(&*self, Self::on_boss_battle_start);
        Message::singleton()
            .signals()
            .boss_battle_end()
            .connect_other(&*self, Self::on_boss_battle_end);
        self.button
            .signals()
            .pressed()
            .connect_other(&*self, Self::on_test);
    }
}

impl PlayerHud {
    fn on_boss_battle_start(&mut self, boss_name: String) {
        self.boss_anim.play_ex().name("show").done();
        self.boss_name.set_text(&boss_name);
        Message::singleton()
            .signals()
            .boss_health_change()
            .connect_other(&*self, Self::on_boss_health_change);
    }
    fn on_boss_battle_end(&mut self) {
        self.boss_anim.play_ex().name("hiden").done();
    }
}

#[godot_api]
impl PlayerHud {
    fn on_boss_health_change(&mut self, hp: f32, max_hp: f32) {
        let hp_precent = (hp / max_hp) as f64;
        self.boss_hp_bar.set_value(hp_precent);
        self.on_hp_hightlight_tween(hp_precent);
    }

    fn on_hp_hightlight_tween(&mut self, to_value: f64) {
        if let Some(mut tween) = self.tween.take() {
            tween.kill();
        }

        let hight_light = self.boss_hp_hightlight.clone();
        let mut tween = self.base_mut().create_tween();
        tween.set_ease(EaseType::IN_OUT);
        tween.set_trans(TransitionType::EXPO);
        tween.tween_interval(0.2);
        tween.tween_property(&hight_light, "value", &to_value.to_variant(), 0.3);

        self.tween = Some(tween);
    }

    fn on_test(&mut self) {
        // Message::singleton().signals().camera_shake().emit(20.0);
        if let Some(node) = self.base().get_tree().get_first_node_in_group("Player")
            && let Ok(player) = node.try_cast::<Node2D>()
        {
            // godot_print!("激发特效");
            // Message::singleton()
            //     .signals()
            //     .play_effect()
            //     .emit(VisualEffectType::Jump, player.get_global_position());
            Message::singleton().signals().play_particles().emit(
                player.get_global_position(),
                Vector2::RIGHT,
                &self.particle_setting.clone(),
            );
        } else {
            godot_print!("没看见有玩家");
        }
    }
    fn on_health_change(&mut self, hp: f32, max_hp: f32) {
        let percent = hp / max_hp;
        // let size = self.hp_container.get_size();
        // godot_print!("血条更新: {hp} / {max_hp} = {percent}");
        // self.hp_container.set_size(size + Vector2::new(22.0, 0.0));
        // godot_print!("{}/{} == {}", hp, max_hp, percent);
        self.hp_bar.set_value(percent as f64);
    }
}
