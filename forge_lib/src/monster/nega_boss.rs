use godot::{
    classes::{AnimationPlayer, Area2D, CharacterBody2D, ICharacterBody2D, Sprite2D},
    init::is_editor_hint,
    prelude::*,
};

use crate::{
    entities::{attack::AttackArea, damage::DamageArea},
    message::Message,
    monster::nega_boss::state_machine::BossStateMachine,
    player::Player,
};

pub(self) mod attack;
pub(self) mod boss_state;
pub(self) mod death;
pub(self) mod hurt;
pub(self) mod idle;
pub(self) mod state_machine;
pub(self) mod walk;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub(crate) struct NegaBoss {
    base: Base<CharacterBody2D>,

    #[init(node = "%PlayerDetector")]
    detector: OnReady<Gd<Area2D>>,

    #[init(node = "%AttackRange")]
    attack_range: OnReady<Gd<Area2D>>,

    #[init(node = "%Sprite2D")]
    body: OnReady<Gd<Sprite2D>>,

    #[init(val = None)]
    target: Option<Gd<Player>>,

    #[init(node = "%AnimationPlayer")]
    anim: OnReady<Gd<AnimationPlayer>>,

    #[export_group(name = "Bind")]
    #[export]
    state_machine: OnEditor<Gd<BossStateMachine>>,
    #[export]
    damage: OnEditor<Gd<DamageArea>>,

    #[export_group(name = "Health")]
    #[export]
    #[init(val = 10.0)]
    hp: f32,

    #[export]
    #[init(val = 10.0)]
    max_hp: f32,

    #[init(val = false)]
    #[var(no_set, get = is_in_attack_range)]
    in_attack_range: bool,
}

#[godot_api]
impl ICharacterBody2D for NegaBoss {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        self.detector
            .signals()
            .body_entered()
            .connect_other(&*self, Self::on_player_entered);
        self.detector
            .signals()
            .body_exited()
            .connect_other(&*self, Self::on_player_exited);
        self.attack_range
            .signals()
            .body_entered()
            .connect_other(&*self, Self::on_range_entered);
        self.attack_range
            .signals()
            .body_exited()
            .connect_other(&*self, Self::on_range_exited);

        Message::singleton()
            .signals()
            .boss_health_change()
            .emit(self.hp, self.max_hp);
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl NegaBoss {
    #[signal]
    pub fn health_change(hp: f32, max_hp: f32);

    #[func]
    fn take_damage(&mut self, _pos: Vector2, _dir: Vector2, a: Gd<AttackArea>) {
        self.hp -= a.bind().get_damage();

        let hp = self.hp;
        let max_hp = self.max_hp;
        self.signals().health_change().emit(hp, max_hp);
        Message::singleton()
            .signals()
            .boss_health_change()
            .emit(self.hp, self.max_hp);
        self.state_machine
            .bind_mut()
            .travel(state_machine::BossState::Hurt);
    }

    pub fn get_health_stats(&self) -> (f32, f32) {
        (self.hp, self.max_hp)
    }

    #[func]
    pub fn set_damage_disable(&mut self, v: bool) {
        self.damage.set_monitoring(!v);
    }

    #[func]
    fn is_in_attack_range(&self) -> bool {
        self.target.is_some() && self.in_attack_range
    }

    fn on_range_entered(&mut self, player: Gd<Node2D>) {
        if let Ok(_player) = player.try_cast::<Player>() {
            self.in_attack_range = true;
        }
    }

    fn on_range_exited(&mut self, player: Gd<Node2D>) {
        if let Ok(_player) = player.try_cast::<Player>() {
            self.in_attack_range = false;
        }
    }

    fn on_player_entered(&mut self, player: Gd<Node2D>) {
        if let Ok(player) = player.try_cast::<Player>() {
            self.target = Some(player);
        }
    }

    fn on_player_exited(&mut self, player: Gd<Node2D>) {
        if player.try_cast::<Player>().is_ok() {
            godot_print!("玩家消失了");
            self.target = None;
        }
    }

    pub fn get_target(&self) -> Option<Gd<Player>> {
        self.target.clone()
    }

    pub fn play_anim(&mut self, anim_name: &str) {
        if self.anim.has_animation(anim_name) {
            self.anim.play_ex().name(anim_name).done();
        }
    }

    pub fn get_anim_length(&self, anim_name: &str) -> f32 {
        if let Some(animation) = self.anim.get_animation(anim_name) {
            // godot_print!("存在动画：{}", anim_name);
            return animation.get_length();
        }
        0.0
    }

    pub fn update_direction(&mut self, new_dir: Vector2) {
        if new_dir.x > 0.0 {
            self.body.set_scale(Vector2::new(1.0, 1.0));
        } else if new_dir.x < 0.0 {
            self.body.set_scale(Vector2::new(-1.0, 1.0));
        }
    }
}
