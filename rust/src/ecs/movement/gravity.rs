use godot::prelude::*;

use crate::ecs::{DynInputControler, EntityCompose, movement::IMovement};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct MovementGravity {
    base: Base<Node>,

    #[export]
    agent: OnEditor<Gd<EntityCompose>>,

    #[export]
    #[init(val = 980.0)]
    gravity: f32,
}

#[godot_dyn]
impl IMovement for MovementGravity {
    fn should_run(&self, input: DynInputControler) -> bool {
        !self.agent.is_on_floor()
    }
    fn run(&mut self, input: DynInputControler, delta: f64) {
        if self.agent.is_on_floor() {
            return;
        }
        let mut velocity = self.agent.get_velocity();
        velocity.y += self.gravity * delta as f32;
        // godot_print!("重力++ {}", velocity);
        self.agent.set_velocity(velocity);
    }
}
