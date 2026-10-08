use godot::{
    classes::{IVisibleOnScreenNotifier2D, VisibleOnScreenNotifier2D, node::ProcessMode},
    prelude::*,
};

use crate::utils::next_tick;

#[derive(GodotClass)]
#[class(init, base = VisibleOnScreenNotifier2D)]
pub(crate) struct InteractObserver {
    base: Base<VisibleOnScreenNotifier2D>,

    #[export]
    only_entered: bool,
}

#[godot_api]
impl IVisibleOnScreenNotifier2D for InteractObserver {
    fn ready(&mut self) {
        self.base()
            .signals()
            .screen_entered()
            .connect_other(&*self, Self::on_screen_entered);
        if !self.only_entered {
            self.base()
                .signals()
                .screen_exited()
                .connect_other(&*self, Self::on_screen_exited);
        }

        next_tick(
            self.base().instance_id(),
            Box::new(|id| {
                if let Ok(node) = Gd::<InteractObserver>::try_from_instance_id(id).as_ref()
                    && let Some(node) = node.get_parent().as_mut()
                {
                    // godot_print!("关闭节点: {}", node.get_name());
                    node.set_process_mode(ProcessMode::DISABLED);
                }
            }),
        );
    }
}

impl InteractObserver {
    fn on_screen_entered(&mut self) {
        if let Some(parent) = self.base().get_parent().as_mut() {
            // godot_print!("打开节点: {}", parent.get_name());
            parent.set_process_mode(ProcessMode::INHERIT);
        }

        if self.only_entered {
            self.base_mut().queue_free();
        }
    }
    fn on_screen_exited(&mut self) {
        if let Some(parent) = self.base().get_parent().as_mut() {
            parent.set_process_mode(ProcessMode::DISABLED);
        }
    }
}
