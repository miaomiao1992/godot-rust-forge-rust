use godot::{global::randf, prelude::*};

use crate::{entities::item_pickup::ItemPickUp, resource::drop_rare::DropRare};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct MagicDrop {
    base: Base<Node>,

    #[export]
    agent: OnEditor<Gd<Node>>,

    #[export]
    #[init(val = Array::default())]
    drop_items: Array<Gd<DropRare>>,

    #[export]
    #[init(val = 10.0)]
    range: f32,
}

#[godot_api]
impl INode for MagicDrop {
    fn ready(&mut self) {
        let node = self.to_gd();
        self.agent
            .connect("death", &Callable::from_object_method(&node, "create_drop"));
    }
}

#[godot_api]
impl MagicDrop {
    #[func]
    fn create_drop(&mut self, center: Vector2) {
        // godot_print!("生成掉落");
        for c in self.drop_items.iter_shared() {
            if randf() < c.bind().rare as f64 {
                let scene_packed = c.bind().item.clone();
                // godot_print!("命中几率");
                for _ in c.bind().min..c.bind().max {
                    let mut drop_item = scene_packed.instantiate_as::<ItemPickUp>();
                    drop_item.set_global_position(center);
                    drop_item.apply_impulse(Vector2::new((randf() as f32) * self.range, -30.0));

                    if let Some(mut root) = self.base().get_tree().get_root() {
                        // godot_print!("添加入tree");
                        root.add_child(&drop_item);
                    } else {
                        // godot_print!("没有场景了");
                    }
                }
            }
        }
    }
}
