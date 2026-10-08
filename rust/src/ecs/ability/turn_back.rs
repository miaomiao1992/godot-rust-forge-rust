use godot::{classes::RayCast2D, prelude::*};

use crate::ecs::{EntityCompose, IInputControler, ability::IAbility};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct AbilityTurnBack {
    base: Base<Node>,
    #[export]
    agent: OnEditor<Gd<EntityCompose>>,
    #[export]
    wall_ray: OnEditor<Gd<RayCast2D>>,
}

#[godot_dyn]
impl IAbility for AbilityTurnBack {
    fn should_run(&self, input: DynGd<Node, dyn IInputControler>) -> bool {
        self.wall_ray.is_colliding() && self.agent.is_on_floor()
    }

    fn run(&mut self, input: DynGd<Node, dyn IInputControler>, delta: f64) {
        let mut velocity = self.agent.get_velocity();
        velocity.x *= -1.0;
        self.agent.set_velocity(velocity);
    }
}
