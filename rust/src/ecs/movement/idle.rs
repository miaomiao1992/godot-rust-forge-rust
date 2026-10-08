use godot::prelude::*;

use crate::ecs::{EntityCompose, IInputControler, movement::IMovement};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct MovementIdle {
    base: Base<Node>,
    #[export]
    agent: OnEditor<Gd<EntityCompose>>,
}

#[godot_dyn]
impl IMovement for MovementIdle {
    fn should_run(&self, input: DynGd<Node, dyn IInputControler>) -> bool {
        input.dyn_bind().get_axis() == 0.0 && self.agent.is_on_floor()
    }
    fn run(&mut self, input: DynGd<Node, dyn IInputControler>, delta: f64) {
        // godot_print!("Movement Idle");
        self.agent.set_velocity(Vector2::ZERO);
    }
}
