use godot::{
    classes::{
        AnimationPlayer, Area2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, Sprite2D,
    },
    init::is_editor_hint,
    prelude::*,
};

use crate::{
    entities::{attack::AttackArea, damage::DamageArea, enery_wave::EnegryWave},
    message::Message,
    monster::nega_boss::state_machine::BossStateMachine,
    player::Player,
};

pub(self) mod attack;
pub(self) mod boss_state;
pub(self) mod dash;
pub(self) mod death;
pub(self) mod fall;
pub(self) mod fly;
pub(self) mod hurt;
pub(self) mod idle;
pub(self) mod jump;
pub(self) mod recover;
pub(self) mod slam;
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

    #[init(node = "%CollisionShape2D")]
    collider: OnReady<Gd<CollisionShape2D>>,

    #[init(val = None)]
    target: Option<Gd<Player>>,

    #[init(node = "%AnimationPlayer")]
    anim: OnReady<Gd<AnimationPlayer>>,

    #[init(val = 1.0)]
    gravity_scale: f32,

    #[export]
    #[init(val = 48.0)]
    height_detect: f32,

    #[export_group(name = "Bind")]
    #[export]
    state_machine: OnEditor<Gd<BossStateMachine>>,
    #[export]
    damage: OnEditor<Gd<DamageArea>>,
    #[export]
    attack: OnEditor<Gd<AttackArea>>,

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

    #[export_group(name = "Dash")]
    #[export]
    #[init(val = 1.0)]
    dash_cool_time: f32,

    #[export]
    #[init(val = 0.0)]
    dashed_time: f32,

    #[export_group(name = "Fly")]
    #[export]
    #[init(val = 5.0)]
    fly_slam_cool_time: f32,

    #[init(val = 0.0)]
    fly_slam_time: f32,

    #[export_group(name = "Enegry")]
    #[export]
    brust: OnEditor<Gd<PackedScene>>,
    #[export]
    wave: OnEditor<Gd<PackedScene>>,
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

    fn physics_process(&mut self, delta: f64) {
        if self.dashed_time > 0.0 {
            self.dashed_time -= delta as f32;
        }

        if self.fly_slam_time > 0.0 {
            self.fly_slam_time -= delta as f32;
        }

        if !self.base().is_on_floor() {
            let mut velocity = self.base().get_velocity();
            velocity += self.gravity_scale * self.base().get_gravity() * delta as f32;
            self.base_mut().set_velocity(velocity);
        }
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl NegaBoss {
    #[signal]
    pub fn health_change(hp: f32, max_hp: f32);

    #[signal]
    pub fn death(pos: Vector2);

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

    #[func]
    pub fn heal_health(&mut self, amount: f32) {
        let hp = self.hp + amount;
        let max_hp = self.max_hp;
        self.hp = hp;
        self.signals().health_change().emit(hp, max_hp);
    }

    pub fn not_in_dash_cooldown(&self) -> bool {
        self.dashed_time <= 0.0
    }

    #[func]
    pub fn set_fly_slam(&mut self) {
        self.fly_slam_time = self.fly_slam_cool_time;
    }

    pub fn is_in_fly_slam_cooldown(&self) -> bool {
        self.fly_slam_time > 0.0
    }

    #[func]
    pub fn set_gravity_scale(&mut self, v: f32) {
        self.gravity_scale = v;
    }

    #[func]
    pub fn set_dash_time(&mut self) {
        self.dashed_time = self.dash_cool_time;
    }

    pub fn get_health_stats(&self) -> (f32, f32) {
        (self.hp, self.max_hp)
    }

    #[func]
    pub fn set_damage_disable(&mut self, v: bool) {
        self.damage.set_monitoring(!v);
    }

    #[func]
    pub fn set_attack_disable(&mut self, v: bool) {
        self.attack.set_monitorable(!v);
    }

    #[func]
    pub fn set_collider_disable(&mut self, v: bool) {
        self.collider.set_disabled(v);
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
            // godot_print!("玩家消失了");
            self.target = None;
        }
    }

    pub fn get_target(&self) -> Option<Gd<Player>> {
        self.target.clone()
    }

    pub fn is_player_on_above_floor(&self) -> bool {
        if let Some(player) = self.target.as_ref()
            && player.is_on_floor()
        {
            if !self.in_attack_range {
                let player_pos = player.get_global_position();
                let pos = self.base().get_global_position();
                if (pos.x - player_pos.x).abs() <= 32.0
                    && (player_pos.y - pos.y).abs() > self.height_detect
                {
                    // godot_print!("检测玩家站在了平台上");
                    return pos.y > player_pos.y;
                }
            }
        }
        false
    }

    pub fn is_player_on_below_floor(&self) -> bool {
        if let Some(player) = self.target.as_ref()
            && player.is_on_floor()
        {
            if !self.in_attack_range {
                let player_pos = player.get_global_position();
                let pos = self.base().get_global_position();

                if (pos.x - player_pos.x).abs() <= 64.0
                    && (player_pos.y - pos.y).abs() > self.height_detect
                {
                    return pos.y < player_pos.y;
                }
            }
        }
        false
    }

    pub fn make_shadow(&mut self) {
        if let Some(mut root) = self.base().get_tree().get_root() {
            let mut shadow_body = self.body.duplicate_node();
            let pos = self.base().get_global_position();
            shadow_body.set_global_position(pos);
            root.add_child(&shadow_body);
            let mut tween = self.base_mut().create_tween();
            // modulate
            let color = self.body.get_modulate();
            tween.tween_property(
                &shadow_body,
                "modulate",
                &color.with_alpha(0.0).to_variant(),
                0.3,
            );
            godot_print!("生成shadow");
            tween.tween_callback(&Callable::from_fn("on_shadow_finished", move |_| {
                godot_print!("销毁Shaodw");
                shadow_body.call_deferred("queue_free", &[]);
            }));
        }
    }

    pub fn create_enegry_wave(&mut self) {
        let pos = self.base().get_global_position();
        if let Some(mut root) = self.base().get_owner() {
            let mut brust_node = self.brust.instantiate_as::<Sprite2D>();
            brust_node.set_global_position(pos);
            root.add_child(&brust_node);

            let mut wave_r = self.wave.instantiate_as::<EnegryWave>();
            wave_r.bind_mut().set_face_to_right(true);
            wave_r.set_global_position(pos);
            root.add_child(&wave_r);

            let mut wave_l = self.wave.instantiate_as::<EnegryWave>();
            wave_l.set_global_position(pos);
            wave_l.set_scale(Vector2::new(-1.0, 1.0));
            wave_l.bind_mut().set_face_to_right(false);
            root.add_child(&wave_l);
        }
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
            self.attack.set_scale(Vector2::new(1.0, 1.0));
        } else if new_dir.x < 0.0 {
            self.body.set_scale(Vector2::new(-1.0, 1.0));
            self.attack.set_scale(Vector2::new(-1.0, 1.0));
        }
    }
}
