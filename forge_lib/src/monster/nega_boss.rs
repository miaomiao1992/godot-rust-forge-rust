use godot::{
    classes::{CharacterBody2D, ICharacterBody2D},
    prelude::*,
};

pub(self) mod boss_state;
pub(self) mod idle;
pub(self) mod state_machine;
pub(self) mod walk;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub(crate) struct NegaBoss {
    base: Base<CharacterBody2D>,
    // #[export]
}

#[godot_api]
impl ICharacterBody2D for NegaBoss {
    fn ready(&mut self) {}
}

#[godot_api]
impl NegaBoss {}
