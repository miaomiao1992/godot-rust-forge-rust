use godot::{obj::WithBaseField, prelude::*};

use crate::{monster::nega_boss::NegaBoss, player::Player};

pub(super) trait IBossState: WithBaseField<Base = Node> {
    fn get_target(&self) -> Option<Gd<Player>> {
        self.get_agent().bind().get_target()
    }

    fn get_agent(&self) -> Gd<NegaBoss> {
        if let Some(owner) = self.base().get_owner()
            && let Ok(boss) = owner.try_cast::<NegaBoss>()
        {
            return boss;
        }
        unreachable!()
    }

    fn play_anim(&self, anim_name: &str) {
        self.get_agent().bind_mut().play_anim(anim_name);
    }

    fn get_anim_length(&self, anim_name: &str) -> f32 {
        self.get_agent().bind().get_anim_length(anim_name)
    }
}
