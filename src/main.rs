use std::collections::HashMap;
use swayipc::reply::Event;
use swayipc::{Connection, EventType, Fallible};

fn get_input_id(c: &mut Connection) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    match c.get_inputs() {
        Ok(inputs) => {
            for i in inputs {
                ids.push(i.identifier);
            }
        }
        _ => {}
    }
    ids
}

fn main() -> Fallible<()> {
    let xdg_dirs = xdg::BaseDirectories::new().unwrap();
    let file = xdg_dirs.place_config_file("swkb.lock").unwrap();
    let instance_a = single_instance::SingleInstance::new(file.to_str().unwrap()).unwrap();
    if !instance_a.is_single() {
        println!("only one instance of swkb at a time is allowed");
        std::process::exit(1);
    }
    let mut connection = Connection::new()?;
    let inputs = get_input_id(&mut connection);
    let mut layouts: HashMap<i64, i64> = HashMap::new();
    let subs = [EventType::Input, EventType::Window];
    let mut events = connection.subscribe(&subs)?;
    let mut current_window: i64 = 0;
    while let Some(event) = events.next() {
        match event {
            Ok(Event::Input(event)) => {
                let layouts_list = event.input.xkb_layout_names;
                let layout_name = event
                    .input
                    .xkb_active_layout_name
                    .unwrap_or("none".to_string());
                if layout_name == "none" {
                    continue;
                }
                let index = layouts_list
                    .iter()
                    .position(|r| *r == layout_name)
                    .unwrap_or(0) as i64;
                layouts.insert(current_window, index);
            }
            Ok(Event::Window(event)) => match event.change {
                swayipc::reply::WindowChange::Focus => {
                    let mut connection = Connection::new()?;
                    for input in &inputs {
                        connection.run_command(format!(
                            "input {} xkb_switch_layout {}",
                            input,
                            layouts
                                .get_key_value(&event.container.id)
                                .unwrap_or((&0, &0))
                                .1
                        ))?;
                    }
                    current_window = event.container.id;
                }
                _ => (),
            },
            _ => (),
        }
    }
    unreachable!();
}
