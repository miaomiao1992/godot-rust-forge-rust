use godot::{obj::WithBaseField, prelude::*};

use crate::ecs::IInputControler;

mod gravity;
mod idle;
mod walk;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct MovementControler {
    base: Base<Node>,
    movements: Array<DynGd<Node, dyn IMovement>>,
}

#[godot_api]
impl INode for MovementControler {
    fn ready(&mut self) {
        for node in self.base().get_children().iter_shared() {
            if node.is_in_group("movement")
                && let Ok(movement) = node.try_dynify::<dyn IMovement>()
            {
                self.movements.push(&movement);
            }
        }
    }
}

#[godot_api]
impl MovementControler {
    pub fn movement_physics(&mut self, input: DynGd<Node, dyn IInputControler>, delta: f64) {
        for mut movement in self.movements.iter_shared() {
            let input = input.clone();
            if movement.dyn_bind().should_run(input.clone()) {
                movement.dyn_bind_mut().run(input, delta);
            }
        }
    }
}

pub(self) trait IMovement {
    fn should_run(&self, input: DynGd<Node, dyn IInputControler>) -> bool {
        false
    }
    fn run(&mut self, input: DynGd<Node, dyn IInputControler>, delta: f64) {}
}
