use godot::{classes::RayCast2D, prelude::*};

use crate::ecs::{DynInputControler, EntityCompose, ability::IAbility};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct AbilityEdgeBack {
    base: Base<Node>,
    #[export]
    agent: OnEditor<Gd<EntityCompose>>,
    #[export]
    edge_ray: OnEditor<Gd<RayCast2D>>,
    #[export]
    disabled: bool,
}

#[godot_dyn]
impl IAbility for AbilityEdgeBack {
    fn should_run(&self, input: DynInputControler) -> bool {
        self.agent.is_on_floor() && !self.edge_ray.is_colliding() && !self.disabled
    }

    fn run(&mut self, input: &mut DynInputControler, delta: f64) {
        let axis = input.dyn_bind().get_axis();
        if axis > 0.0 {
            input.dyn_bind_mut().set_left_held(true);
            input.dyn_bind_mut().set_right_held(false);
        } else if axis < 0.0 {
            input.dyn_bind_mut().set_left_held(false);
            input.dyn_bind_mut().set_right_held(true);
        }
        input.dyn_bind_mut().set_axis(-axis);
    }
}
