use godot::{classes::Engine, prelude::*};

pub(crate) fn next_tick(id: InstanceId, callback: Box<dyn FnOnce(InstanceId)>) {
    godot::task::spawn(async move {
        if let Some(main) = Engine::singleton().get_main_loop()
            && let Ok(tree) = main.try_cast::<SceneTree>()
        {
            let next_frame = tree.signals().process_frame().to_future();
            next_frame.await;

            callback(id);
        }
    });
}

#[allow(dead_code)]
pub(crate) fn process_frame(id: InstanceId, method_name: &str, args: &[Variant]) {
    let method_name = method_name.to_owned();
    let args = args.to_owned();
    godot::task::spawn(async move {
        if let Some(main) = Engine::singleton().get_main_loop()
            && let Ok(tree) = main.try_cast::<SceneTree>()
        {
            let next_frame = tree.signals().process_frame().to_future();
            next_frame.await;

            if let Ok(mut node) = Gd::<Object>::try_from_instance_id(id) {
                if node.has_method(&method_name) {
                    node.call(&method_name, &args);
                }
            }
        }
    });
}

#[allow(dead_code)]
pub(crate) fn delay(secs: f64, id: InstanceId, callback: Box<dyn FnOnce(InstanceId)>) {
    godot::task::spawn(async move {
        if let Some(main) = Engine::singleton().get_main_loop()
            && let Ok(tree) = main.try_cast::<SceneTree>().as_mut()
        {
            let timeout = tree.create_timer(secs).signals().timeout().to_future();
            timeout.await;
            callback(id);
        }
    });
}
