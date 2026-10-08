use godot::prelude::*;

use crate::ecs::{DynInputControler, EntityCompose, movement::IMovement};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct MovementWalk {
    base: Base<Node>,
    #[export]
    agent: OnEditor<Gd<EntityCompose>>,

    #[export]
    #[init(val = 100.0)]
    speed: f32,
}

#[godot_dyn]
impl IMovement for MovementWalk {
    fn should_run(&self, input: DynInputControler) -> bool {
        input.dyn_bind().get_axis() != 0.0
            && (input.dyn_bind().get_right_held() || input.dyn_bind().get_left_held())
        // && self.agent.is_on_floor()
    }
    fn run(&mut self, input: DynInputControler, delta: f64) {
        let mut velocity = self.agent.get_velocity();

        let axis = input.dyn_bind().get_axis();

        if axis > 0.0 {
            velocity.x = self.speed;
        } else if axis < 0.0 {
            velocity.x = -self.speed;
        }

        self.agent.set_velocity(velocity);
    }
}
