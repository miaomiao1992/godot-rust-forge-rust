use godot::prelude::*;

use crate::ecs::{DynInputControler, EntityCompose, ability::IAbility};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct AbilityJump {
    base: Base<Node>,
    #[export]
    agent: OnEditor<Gd<EntityCompose>>,

    #[export]
    #[init(val = 200.0)]
    jumping_stength: f32,
}

#[godot_dyn]
impl IAbility for AbilityJump {
    fn should_run(&self, input: DynInputControler) -> bool {
        input.dyn_bind().get_jump_pressed() && self.agent.is_on_floor()
    }

    fn run(&mut self, input: &mut DynInputControler, delta: f64) {
        let mut velocity = self.agent.get_velocity();
        velocity.y -= self.jumping_stength;
        self.agent.set_velocity(velocity);
    }
}
