use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossWalk {
    base: Base<Node>,
}

impl IBossState for BossWalk {}

#[godot_api]
impl BossWalk {
    #[func]
    fn entered(&mut self) {
        godot_print!("进去walk");
    }

    #[func]
    fn exited(&mut self) {
        godot_print!("退出walk");
    }
}
