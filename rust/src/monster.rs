use godot::{
    classes::{
        AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D, Marker2D, RayCast2D, Timer,
    },
    init::is_editor_hint,
    prelude::*,
    signal::ConnectHandle,
};

use crate::{
    entities::{
        attack::AttackArea, bullet::Bullet, damage::DamageArea, edge_detector::EdgeDetector,
    },
    player::Player,
};
//ai base
pub(crate) mod blackboard;
pub(crate) mod decision_engine;
pub(crate) mod enemy;
pub(crate) mod enemy_state;
pub(crate) mod enemy_state_machine;

//monsters
pub(crate) mod slime;
pub(crate) mod slime_standard;

pub(crate) mod nega_boss;

#[derive(GodotConvert, Default, Export, Var, Debug, Clone, Copy, PartialEq, Eq)]
#[godot(via = GString)]
pub(crate) enum AttackType {
    RANGE,
    MELEE,
    #[default]
    NONE,
}

#[derive(GodotConvert, Default, Export, Var, Debug, Clone, Copy, PartialEq, Eq)]
#[godot(via = GString)]
pub(crate) enum Vigilance {
    Hight,
    #[default]
    Low,
}

#[derive(GodotConvert, Default, Export, Var, Debug, Clone, Copy, PartialEq, Eq)]
#[godot(via = GString)]
pub(crate) enum DirAxios {
    #[default]
    Left,
    Right,
}

impl TryFrom<f32> for DirAxios {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        match value {
            a @ ..0.0 if a < 0.0 => Ok(DirAxios::Left),
            0.0.. => Ok(DirAxios::Right),
            n => Err(format!("无法转换的数：{}", n)),
        }
    }
}

#[derive(GodotClass)]
#[class(init, tool, base = CharacterBody2D)]
pub(crate) struct Monster {
    base: Base<CharacterBody2D>,

    #[export]
    is_show_log: bool,

    #[export]
    #[init(val = 5.0)]
    max_hp: f32,

    #[init(val = 0.0)]
    hp: f32,

    #[export]
    start_axios: DirAxios,

    #[export]
    guard: Vigilance,

    #[export]
    #[init(val = 1.0)]
    #[var(set, get)]
    dir: f32,

    #[export]
    patrol: bool,
    #[export]
    #[init(val = 800.0)]
    patrol_speed: f32,

    #[export_group(name = "tools")]
    #[export]
    body: OnEditor<Gd<AnimatedSprite2D>>,
    #[export]
    sight: OnEditor<Gd<Area2D>>,
    #[export]
    sight_ray: OnEditor<Gd<RayCast2D>>,
    #[export]
    damage_area: OnEditor<Gd<DamageArea>>,
    #[export]
    search_timer: Option<Gd<Timer>>,

    #[export_group(name = "Attack")]
    #[export]
    attack_type: AttackType,
    #[export_subgroup(name = "melee")]
    #[export]
    #[init(val = 3000.0)]
    melee_chase_speed: f32,
    #[export]
    melee_auto_attack: bool,
    #[export]
    #[init(val = 60.0)]
    melee_range: f32,
    #[export]
    #[init(val = Vector2i::new(6, 8))]
    attack_frame_range: Vector2i,

    #[export_subgroup(name = "range")]
    #[export]
    #[init(val = None)]
    fire_marker: Option<Gd<Marker2D>>,
    #[export]
    #[init(val = 1.0)]
    shoot_delay: f32,
    #[export]
    bullet: Option<Gd<PackedScene>>,

    shoot_time: f32,
    attacking: bool,
    hurting: bool,
    target: Option<Gd<Node2D>>,
    connect_handle: Option<ConnectHandle>,
}

#[godot_api]
impl ICharacterBody2D for Monster {
    fn ready(&mut self) {
        if is_editor_hint() {
            self.base_mut().set_physics_process(false);
            return;
        }
        self.hp = self.max_hp;

        self.sight
            .signals()
            .body_entered()
            .connect_other(&*self, Self::on_player_entered);
        self.sight
            .signals()
            .body_exited()
            .connect_other(&*self, Self::on_player_exited);
        self.connect_handle = Some(
            self.body
                .signals()
                .animation_finished()
                .connect_other(&*self, Self::on_shoot_animation_finished),
        );

        if self.patrol {
            if let Some(edge_detector) =
                self.base().try_get_node_as::<EdgeDetector>("%EdgeDetector")
            {
                // godot_print!("检测边缘： {}", self.base().get_name());
                edge_detector
                    .signals()
                    .edge_detect()
                    .connect_other(&*self, Self::on_edge_detected);
            }

            self.body.play_ex().name("run").done();
        }

        self.set_melee_enable(false);

        if let Some(timer) = self.search_timer.as_ref() {
            timer
                .signals()
                .timeout()
                .connect_other(&*self, Self::on_searching_finished);
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let delta = delta as f32;
        if self.hurting {
            self.try_knockback();
            return;
        }
        if self.attacking && !self.is_sight_blocked() {
            if self.attack_type == AttackType::RANGE {
                //执行远程攻击
                self.shoot_time -= delta;
                if self.shoot_time <= 0.0 {
                    self.shoot_time = self.shoot_delay;
                    self.shoot();
                }
                return;
            } else if self.can_melee_attack() {
                //执行近战攻击
                self.make_melee();
                return;
            } else if self.attack_type == AttackType::MELEE {
                //chase player
                self.melee_chase(delta);
                self.base_mut().move_and_slide();
                return;
            }
        }

        if self.patrol {
            //执行巡逻
            self.body.play_ex().name("run").done();
            let dir = self.get_walk_dir();
            self.set_dir(dir.x);
            let velocity = dir * self.patrol_speed * delta * Vector2::RIGHT;
            self.base_mut().set_velocity(velocity);
        } else {
            //idle
            self.body.play_ex().name("idle").done();
            self.base_mut().set_velocity(Vector2::ZERO);
        }

        self.apply_gravity(delta);

        self.base_mut().move_and_slide();

        if self.base().is_on_wall() {
            let dir = self.get_walk_dir();
            self.set_dir(-dir.x);
        }
    }
}

#[godot_api]
impl Monster {
    #[signal]
    pub fn death(pos: Vector2);

    fn on_edge_detected(&mut self) {
        self.attacking = false;
        self.set_dir(-self.dir);
    }

    fn on_searching_finished(&mut self) {
        // godot_print!("搜寻玩家结束");
        self.on_player_miss_completed();
    }

    fn apply_gravity(&mut self, delta: f32) {
        if !self.base().is_on_floor() {
            let gravity = self.base().get_gravity();
            let mut velocity = self.base().get_velocity();
            velocity.x = 0.0;
            velocity.y += gravity.y * delta;

            self.base_mut().set_velocity(velocity);
        }
    }

    #[func]
    pub fn take_damage(&mut self, _pos: Vector2, _dir: Vector2, from: Gd<AttackArea>) {
        self.hp -= from.bind().get_damage();
        self.set_melee_enable(false);
        self.body.play_ex().name("hit").done();
        // godot_print!("{} 被打了", self.base().get_name());
        self.damage_area.set_monitoring(false);
        self.hurting = true;
        self.attacking = true;
    }

    fn try_knockback(&mut self) {
        if let Some(target) = self.target.as_ref()
            && self.patrol
        {
            let target_pos = target.get_global_position();
            let pos = self.base().get_global_position();
            let dir = target_pos.direction_to(pos);

            let speed = self.patrol_speed;
            self.base_mut().set_velocity(dir * speed);
        }
    }

    fn is_sight_blocked(&self) -> bool {
        if let Some(collider) = self.sight_ray.get_collider()
            && collider.try_cast::<Player>().is_err()
        {
            return true;
        }

        false
    }

    fn get_walk_dir(&self) -> Vector2 {
        if self.start_axios == DirAxios::try_from(self.dir).unwrap_or_default() {
            return Vector2::new(self.dir, 0.0);
        }
        let init_face = match self.start_axios {
            DirAxios::Left => 1.0,
            DirAxios::Right => -1.0,
        };

        Vector2::new(init_face, 0.0)
    }

    pub fn set_melee_enable(&mut self, v: bool) {
        if self.attack_type == AttackType::MELEE {
            if let Some(mut attack_area) = self.base().try_get_node_as::<AttackArea>("%AttackArea")
            {
                attack_area.call_deferred("set_monitorable", &[v.to_variant()]);
                attack_area.call_deferred("set_visible", &[v.to_variant()]);
            }
        }
    }

    #[func]
    pub fn get_dir(&self) -> f32 {
        self.dir
    }

    #[func]
    pub fn set_dir(&mut self, v: f32) {
        self.dir = v;
        if self.start_axios == DirAxios::try_from(v).unwrap_or_default() {
            self.body.set_scale(Vector2::new(1.0, 1.0));
        } else {
            self.body.set_scale(Vector2::new(-1.0, 1.0));
        }
    }

    fn on_player_entered(&mut self, body: Gd<Node2D>) {
        self.target = Some(body.clone());
        // godot_print!(
        //     "Monster: {}, 发现玩家, 攻击类型: {:?}",
        //     self.base().get_name(),
        //     self.attack_type
        // );
        if let Some(timer) = self.search_timer.as_mut() {
            timer.stop();
        }

        match self.attack_type {
            AttackType::RANGE => {
                self.range_attack(body);
            }
            AttackType::MELEE => {
                self.melee_attack(body);
            }
            _ => {}
        }
    }

    fn melee_attack(&mut self, _body: Gd<Node2D>) {
        self.attacking = true;
    }

    fn can_melee_attack(&self) -> bool {
        if let Some(body) = self.target.as_ref()
            && self.attack_type == AttackType::MELEE
        {
            let target_pos = body.get_global_position();
            let pos = self.base().get_global_position();

            let walk_dir = self.get_walk_dir();
            let dir = pos.direction_to(target_pos);
            let is_same_dir = walk_dir.dot(dir) > 0.0;

            return (target_pos.x - pos.x).abs() < self.melee_range && is_same_dir;
        }
        false
    }

    fn melee_chase(&mut self, delta: f32) {
        if self.melee_auto_attack
            && let Some(body) = self.target.as_ref()
        {
            self.body.play_ex().name("run").done();

            let target_pos = body.get_global_position();
            let pos = self.base().get_global_position();

            let dir = pos.direction_to(target_pos) * Vector2::RIGHT;
            self.set_dir(dir.x);
            let speed = self.melee_chase_speed * dir * delta;
            self.base_mut().set_velocity(speed);
        }

        self.set_melee_enable(false);
    }

    fn make_melee(&mut self) {
        if let Some(body) = self.target.as_ref() {
            let target_pos = body.get_global_position();
            let pos = self.base().get_global_position();

            let dir = pos.direction_to(target_pos) * Vector2::RIGHT;
            self.set_dir(dir.x);
        }
        self.body.play_ex().name("attack").done();
        self.base_mut().set_velocity(Vector2::ZERO);
        let current_frame = self.body.get_frame();

        if current_frame >= self.attack_frame_range.x && current_frame <= self.attack_frame_range.y
        {
            self.set_melee_enable(true);
        } else {
            self.set_melee_enable(false);
        }
    }

    fn on_player_exited(&mut self, _body: Gd<Node2D>) {
        match self.guard {
            Vigilance::Hight if self.search_timer.is_some() => {
                // godot_print!("开始寻找");
                let timer = self.search_timer.as_mut().unwrap();
                timer.start();
            }
            _ => {
                self.on_player_miss_completed();
            }
        }
    }

    fn on_player_miss_completed(&mut self) {
        self.attacking = false;
        self.target = None;
        self.set_melee_enable(false);
    }

    fn range_attack(&mut self, _target: Gd<Node2D>) {
        self.attacking = true;
        self.shoot_time = self.shoot_delay;
    }

    fn shoot(&mut self) {
        // godot_print!("SHOOT");
        self.body.play_ex().name("attack").done();
    }

    fn on_shoot_animation_finished(&mut self) {
        if self.hp <= 0.0 {
            // godot_print!("死球了");
            let pos = self.base().get_global_position();
            self.signals().death().emit(pos);
            self.base_mut().call_deferred("queue_free", &[]);
            return;
        }

        if self.body.get_animation() == "attack" {
            // godot_print!("动画播放完成");
            if self.attack_type == AttackType::RANGE {
                if let Some(bullet_scene) = self.bullet.as_ref()
                    && let Some(mut bullet) = bullet_scene.try_instantiate_as::<Bullet>()
                {
                    let dir = self.get_walk_dir();
                    bullet.bind_mut().set_dir(dir);
                    let fire_pos = self
                        .fire_marker
                        .as_ref()
                        .map_or(Vector2::ZERO, |node| node.get_global_position());
                    bullet.set_global_position(fire_pos);

                    if let Some(root) = self.base().get_tree().get_root().as_mut() {
                        root.add_child(&bullet);
                    }
                }
            }

            self.set_melee_enable(false);
        } else if self.body.get_animation() == "hit" {
            // godot_print!("HIT OK FINISHED");
            self.hurting = false;
            self.damage_area.set_monitoring(true);
        }

        if self.patrol {
            self.body.play_ex().name("run").done();
        } else {
            self.body.play_ex().name("idle").done();
        }
    }
}
