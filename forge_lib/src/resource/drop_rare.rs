use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(init, base = Resource)]
pub(crate) struct DropRare {
    base: Base<Resource>,

    #[export(range = (0.0, 1.0, 0.01))]
    #[init(val = 0.3)]
    pub rare: f32,

    #[export]
    #[init(val = 1)]
    pub min: i32,

    #[export]
    #[init(val = 1)]
    pub max: i32,

    #[export]
    pub item: OnEditor<Gd<PackedScene>>,
}
