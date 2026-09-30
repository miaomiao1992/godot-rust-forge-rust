use godot::prelude::*;

use crate::monster::nega_boss::boss_state::IBossState;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct BossIdle {
    base: Base<Node>,
    #[init(val = 5.0)]
    duration: f32,
}

impl IBossState for BossIdle {}

#[godot_api]
impl BossIdle {
    #[func]
    fn entered(&mut self) {
        godot_print!("进去idle");
    }

    #[func]
    fn exited(&mut self) {
        godot_print!("退出idle");
    }

    #[func]
    fn update(&mut self, delta: f64) -> Variant {
        let delta = delta as f32;
        self.duration -= delta;

        if self.duration <= 0.0 {
            return "Walk".to_variant();
        }

        Variant::nil()
    }
}
