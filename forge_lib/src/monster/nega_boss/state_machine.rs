use godot::{
    classes::{InputEvent, node::ProcessMode},
    init::is_editor_hint,
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct BossStateMachine {
    base: Base<Node>,
    #[init(val = None)]
    current_state: Option<Gd<Node>>,

    #[export]
    #[init(val = BossState::default())]
    init_state: BossState,
}

#[godot_api]
impl INode for BossStateMachine {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }
        let next_state = self.init_state.clone();
        self.base_mut()
            .call_deferred("travel", &[next_state.to_variant()]);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(state) = self.current_state.as_mut() {
            if state.has_method("handle_input") {
                let next_state = state.call("handle_input", &[event.to_variant()]);
                if next_state.is_nil() {
                    return;
                }

                let state_name = String::from_variant(&next_state);
                self.travel(BossState::from_godot(state_name.to_godot()));
            }
        }
    }

    fn process(&mut self, delta: f64) {
        if let Some(state) = self.current_state.as_mut() {
            if state.has_method("update") {
                let next_state = state.call("update", &[delta.to_variant()]);
                if next_state.is_nil() {
                    return;
                }

                let state_name = String::from_variant(&next_state);
                self.travel(BossState::from_godot(state_name.to_godot()));
            }
        }
    }
}

#[godot_api]
impl BossStateMachine {
    #[func]
    pub fn travel(&mut self, next_state: BossState) {
        self.base_mut().set_process_mode(ProcessMode::DISABLED);
        if let Some(mut current_state) = self.current_state.take() {
            current_state.try_call("exited", &[]).ok();
        }

        let id = self.base().instance_id();
        let one_frame_after = self.base().get_tree().signals().process_frame().to_future();

        godot::task::spawn(async move {
            one_frame_after.await;
            if let Ok(mut machine) = Gd::<BossStateMachine>::try_from_instance_id(id) {
                machine.bind_mut().enter_next_state(next_state);
            }
        });
    }

    pub fn enter_next_state(&mut self, next_state: BossState) {
        if let Some(mut state_node) = self.get_state_node(next_state) {
            state_node.try_call_deferred("entered", &[]).ok();
            self.current_state = Some(state_node);
        }
        self.base_mut().set_process_mode(ProcessMode::INHERIT);
    }

    fn get_state_node(&self, state: BossState) -> Option<Gd<Node>> {
        let state_node_name: &str = state.into();
        self.base().try_get_node_as(state_node_name)
    }
}

#[derive(GodotConvert, Debug, Default, Clone, Copy, Var, Export)]
#[godot(via = GString)]
pub(crate) enum BossState {
    #[default]
    Idle,
    Walk,
    Attack,
    Death,
}

impl From<BossState> for &str {
    fn from(value: BossState) -> Self {
        match value {
            BossState::Idle => "Idle",
            BossState::Walk => "Walk",
            BossState::Attack => "Attack",
            BossState::Death => "Death",
        }
    }
}
