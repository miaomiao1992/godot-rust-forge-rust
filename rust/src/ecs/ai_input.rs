use godot::{classes::RayCast2D, prelude::*};

use crate::ecs::IInputControler;

#[derive(GodotConvert, Debug, Default, Clone, Export, Var)]
#[godot(via = GString)]
pub enum AIInputState {
    #[default]
    Idle,
    Walk,
    Jump,
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AIInputControler {
    base: Base<Node>,

    #[export]
    disabled: bool,
    #[export]
    wall_ray: OnEditor<Gd<RayCast2D>>,
    #[export]
    edge_ray: OnEditor<Gd<RayCast2D>>,
    #[export]
    jump_ray: OnEditor<Gd<RayCast2D>>,

    #[export]
    state: AIInputState,

    #[export]
    #[init(val = 5.0)]
    idle_duration: f32,
    #[export]
    #[init(val = 5.0)]
    walk_duration: f32,
    #[export]
    #[init(val = 1.0)]
    start_axis: f32,

    axis: f32,
    left_held: bool,
    right_held: bool,
    jump_pressed: bool,
}

#[godot_api]
impl INode for AIInputControler {
    fn ready(&mut self) {
        self.axis = self.start_axis;
        if self.start_axis > 0.0 {
            self.right_held = true;
            self.state = AIInputState::Walk;
        } else if self.start_axis < 0.0 {
            self.left_held = true;
            self.state = AIInputState::Walk;
        }
        self.update_dir();
    }
}

impl AIInputControler {
    fn update_dir(&mut self) {
        let v = self.axis;
        if v > 0.0 {
            self.wall_ray.set_scale(Vector2::new(1.0, 1.0));
            self.edge_ray.set_scale(Vector2::new(1.0, 1.0));
        } else if v < 0.0 {
            self.wall_ray.set_scale(Vector2::new(-1.0, 1.0));
            self.edge_ray.set_scale(Vector2::new(-1.0, 1.0));
        }
    }
}

#[godot_dyn]
impl IInputControler for AIInputControler {
    fn set_axis(&mut self, v: f32) {
        self.axis = v;
        self.update_dir();
    }
    fn get_axis(&self) -> f32 {
        self.axis
    }

    fn set_left_held(&mut self, v: bool) {
        self.left_held = v;
    }
    fn get_left_held(&self) -> bool {
        self.left_held
    }

    fn set_right_held(&mut self, v: bool) {
        self.right_held = v;
    }

    fn get_right_held(&self) -> bool {
        self.right_held
    }

    fn get_jump_pressed(&self) -> bool {
        self.jump_pressed
    }

    fn input_physics(&mut self, delta: f64) {
        self.jump_pressed = false;
        if self.wall_ray.is_colliding() {
            if !self.jump_ray.is_colliding() && !self.jump_pressed {
                self.jump_pressed = true;
            }
        }
    }
}
