use godot::{classes::Input, prelude::*};

use crate::ecs::IInputControler;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct InputControler {
    base: Base<Node>,

    axis: f32,
    jump_pressed: bool,
    attack_pressed: bool,
    interact_pressed: bool,
    left_held: bool,
    right_held: bool,

    #[export]
    disabled: bool,
}

#[godot_dyn]
impl IInputControler for InputControler {
    fn get_axis(&self) -> f32 {
        self.axis
    }
    fn get_jump_pressed(&self) -> bool {
        self.jump_pressed
    }
    fn get_attack_pressed(&self) -> bool {
        self.attack_pressed
    }
    fn get_interact_pressed(&self) -> bool {
        self.interact_pressed
    }
    fn get_left_held(&self) -> bool {
        self.left_held
    }
    fn get_right_held(&self) -> bool {
        self.right_held
    }

    fn input_physics(&mut self, _delta: f64) {
        if self.disabled {
            self.axis = 0.0;
            self.jump_pressed = false;
            self.attack_pressed = false;
            self.interact_pressed = false;
            self.left_held = false;
            self.right_held = false;
            return;
        }
        let input = Input::singleton();
        self.axis = input.get_axis("left", "right");
        self.jump_pressed = input.is_action_just_pressed("jump");
        self.attack_pressed = input.is_action_just_pressed("attack");
        self.interact_pressed = input.is_action_just_pressed("action");
        self.left_held = input.is_action_pressed("left");
        self.right_held = input.is_action_pressed("right");
    }
}

#[godot_api]
impl INode for InputControler {
    fn to_string(&self) -> GString {
        format!(
            "axis: {}, jump: {}, attack: {}, interact: {}, left_held: {}, right_held: {}",
            self.axis,
            self.jump_pressed,
            self.attack_pressed,
            self.interact_pressed,
            self.left_held,
            self.right_held
        )
        .to_gstring()
    }
}
