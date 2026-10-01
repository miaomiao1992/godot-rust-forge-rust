use godot::{
    global::{randf, randf_range},
    prelude::*,
};

use crate::{
    entities::item_pickup::ItemPickUp, monster::nega_boss::NegaBoss, resource::drop_rare::DropRare,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct MagicDrop {
    base: Base<Node>,

    #[export]
    agent: OnEditor<Gd<NegaBoss>>,

    #[export]
    #[init(val = Array::default())]
    drop_items: Array<Gd<DropRare>>,
}

#[godot_api]
impl INode for MagicDrop {
    fn ready(&mut self) {
        self.agent
            .signals()
            .health_change()
            .connect_other(&*self, Self::on_agent_exiting);
    }
}

impl MagicDrop {
    fn on_agent_exiting(&mut self, hp: f32, _max_hp: f32) {
        if hp <= 0.0 {
            let pos = self.agent.get_global_position();
            self.create_drop(pos);
        }
    }

    fn create_drop(&mut self, center: Vector2) {
        godot_print!("生成掉落");
        for c in self.drop_items.iter_shared() {
            if randf() < c.bind().rare as f64 {
                let scene_packed = c.bind().item.clone();
                godot_print!("命中几率");
                for _ in c.bind().min..c.bind().max {
                    let mut drop_item = scene_packed.instantiate_as::<ItemPickUp>();
                    drop_item.set_global_position(center);
                    drop_item.apply_impulse(Vector2::new(randf_range(-10.0, 10.0) as f32, -30.0));

                    if let Some(mut root) = self.base().get_tree().get_root() {
                        root.add_child(&drop_item);
                    }
                }
            }
        }
    }
}
