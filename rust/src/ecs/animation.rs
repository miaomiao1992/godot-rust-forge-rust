use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AnimationControler {
    base: Base<Node>,
}

#[godot_api]
impl AnimationControler {
    pub fn animation_physics(&mut self, delta: f64) {}
}
