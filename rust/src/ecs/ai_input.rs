use godot::{classes::RayCast2D, global::randf, prelude::*};

use crate::ecs::IInputControler;

#[derive(GodotConvert, Debug, Default, Clone, Export, Var)]
#[godot(via = GString)]
pub enum AIInputState {
    #[default]
    Idle,
    WalkRight,
    WalkLeft,
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
    state: AIInputState,

    #[export]
    #[init(val = 5.0)]
    idle_duration: f32,
    #[export]
    #[init(val = 5.0)]
    walk_duration: f32,

    axis: f32,
    time: f32,
    left_held: bool,
    right_held: bool,
}

// impl INode for AIInputControler {
//     fn ready(&mut self,) {
//     }
// }

#[godot_dyn]
impl IInputControler for AIInputControler {
    fn get_axis(&self) -> f32 {
        self.axis
    }

    fn get_left_held(&self) -> bool {
        self.left_held
    }

    fn get_right_held(&self) -> bool {
        self.right_held
    }

    fn input_physics(&mut self, delta: f64) {
        let delta = delta as f32;

        match self.state {
            AIInputState::Idle => {
                self.axis = 0.0;
                self.left_held = false;
                self.right_held = false;
                self.time += delta;

                if self.time >= self.idle_duration {
                    self.time = 0.0;
                    self.state = if randf() > 0.5 {
                        AIInputState::WalkRight
                    } else {
                        AIInputState::WalkLeft
                    };
                }
            }
            AIInputState::WalkRight => {
                self.axis = 1.0;
                self.time += delta;
                self.right_held = true;

                if self.time >= self.idle_duration {
                    self.time = 0.0;
                    self.state = AIInputState::Idle;
                }

                if self.wall_ray.is_colliding() {
                    self.axis = -1.0;
                    self.state = AIInputState::WalkLeft;
                }
            }
            AIInputState::WalkLeft => {
                self.axis = -1.0;
                self.time += delta;
                self.left_held = true;

                if self.time >= self.idle_duration {
                    self.time = 0.0;
                    self.state = AIInputState::Idle;
                }

                if self.wall_ray.is_colliding() {
                    self.axis = 1.0;
                    self.state = AIInputState::WalkRight;
                }
            }
            _ => {}
        }
    }
}
