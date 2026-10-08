use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, RayCast2D, Sprite2D},
    prelude::*,
};

use crate::ecs::{
    ability::AbilityControler, animation::AnimationControler, movement::MovementControler,
};

pub mod ability;
pub mod ai_input;
pub mod animation;
pub mod input;
pub mod movement;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub struct EntityCompose {
    base: Base<CharacterBody2D>,
    #[export]
    input: OnEditor<DynGd<Node, dyn IInputControler>>,
    #[export]
    wall_ray: OnEditor<Gd<RayCast2D>>,
    #[export]
    sprite: OnEditor<Gd<Sprite2D>>,

    #[export]
    movement: OnEditor<Gd<MovementControler>>,
    #[export]
    ability: OnEditor<Gd<AbilityControler>>,
    #[export]
    animation: OnEditor<Gd<AnimationControler>>,
}

#[godot_api]
impl ICharacterBody2D for EntityCompose {
    fn process(&mut self, _delta: f64) {
        let axis = self.input.dyn_bind().get_axis();
        if axis > 0.0 {
            self.sprite.set_scale(Vector2::new(1.0, 1.0));
            self.wall_ray.set_scale(Vector2::new(1.0, 1.0));
        } else if axis < 0.0 {
            self.sprite.set_scale(Vector2::new(-1.0, 1.0));
            self.wall_ray.set_scale(Vector2::new(-1.0, 1.0));
        }
    }
    fn physics_process(&mut self, delta: f64) {
        self.input.dyn_bind_mut().input_physics(delta);
        {
            let input = self.input.clone();
            self.movement.bind_mut().movement_physics(input, delta);
        }
        {
            let input = self.input.clone();
            let movement = self.movement.clone();
            self.ability
                .bind_mut()
                .ability_physics(input, movement, delta);
        }
        self.animation.bind_mut().animation_physics(delta);

        self.base_mut().move_and_slide();
    }
}

pub trait IInputControler {
    fn get_axis(&self) -> f32 {
        0.0
    }
    fn get_jump_pressed(&self) -> bool {
        false
    }
    fn get_attack_pressed(&self) -> bool {
        false
    }
    fn get_interact_pressed(&self) -> bool {
        false
    }
    fn get_left_held(&self) -> bool {
        false
    }
    fn get_right_held(&self) -> bool {
        false
    }

    fn input_physics(&mut self, delta: f64);
}
