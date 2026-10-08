use godot::prelude::*;

use crate::ecs::{DynInputControler, movement::MovementControler};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AbilityControler {
    base: Base<Node>,
    abilities: Array<DynGd<Node, dyn IAbility>>,
}

mod edge_back;
mod jump;
mod turn_back;

#[godot_api]
impl AbilityControler {
    pub fn ability_physics(
        &mut self,
        input: &mut DynInputControler,
        movement: Gd<MovementControler>,
        delta: f64,
    ) {
        for mut ability in self.abilities.iter_shared() {
            let mut input = input.clone();
            if ability.dyn_bind().should_run(input.clone()) {
                ability.dyn_bind_mut().run(&mut input, delta);
            }
        }
    }
}
#[godot_api]
impl INode for AbilityControler {
    fn ready(&mut self) {
        for node in self.base().get_children().iter_shared() {
            if node.is_in_group("ability")
                && let Ok(movement) = node.try_dynify::<dyn IAbility>()
            {
                self.abilities.push(&movement);
            }
        }
    }
}

pub(self) trait IAbility {
    fn should_run(&self, input: DynInputControler) -> bool {
        false
    }
    fn run(&mut self, input: &mut DynInputControler, delta: f64) {}
}
