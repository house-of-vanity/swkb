use async_std::stream::StreamExt;
use std::collections::HashMap;
use std::sync::Mutex;
use swayipc::reply::Event;
use swayipc::{Connection, EventType, Fallible};


async fn get_input_id() -> Vec<String> {
    let mut connection = Connection::new().await.unwrap();
    let inputs = connection.get_inputs().await.unwrap();
    let mut ids: Vec<String> = Vec::new();
    for i in inputs {
      ids.push(i.identifier);
    }
    ids
}

async fn get_focus_id() -> i64 {
    let mut connection = Connection::new().await.unwrap();
    let tree = connection.get_tree().await.unwrap();
    let mut focused: i64 = 0;
    for i in tree.nodes {
        for z in i.nodes {
            for x in z.nodes {
                if x.focused == true {
                    focused = x.id;
                    /*
                    println!(
                        "Focused window [id {:?}] {:?}",
                        x.id,
                        x.name.unwrap_or("unnamed".to_string())
                    );
                    */
                }
            }
        }
    }
    focused
}

#[async_std::main]
async fn main() -> Fallible<()> {
    let inputs = get_input_id().await;
    let connection = Connection::new().await?;
    let layouts: Mutex<HashMap<i64, i64>> = Mutex::new(HashMap::new());
    let subs = [EventType::Input, EventType::Window];
    let mut events = connection.subscribe(&subs).await?;
    while let Some(event) = events.next().await {
        match event.unwrap() {
            Event::Input(event) => {
                let layouts_list = event.input.xkb_layout_names;
                let layout_name = event.input.xkb_active_layout_name.unwrap();
                let index = layouts_list.iter().position(|r| *r == layout_name).unwrap() as i64;
                let mut layouts = layouts.lock().unwrap();
                let current_window = get_focus_id().await;
                //println!("Layout saved [{:?}] for {:?}", layout_name, current_window);
                layouts.insert(current_window, index);
            }
            Event::Window(event) => match event.change {
                swayipc::reply::WindowChange::Focus => {
                    let layouts = layouts.lock().unwrap();
                    let mut connection = Connection::new().await?;
                    /*
                    println!(
                        "Restoring layout [{:?}] for {:?}",
                        layouts
                            .get_key_value(&event.container.id)
                            .unwrap_or((&0, &0))
                            .1,
                        event.container.id
                    );
                    */
                    for input in &inputs {
                    connection
                        .run_command(format!(
                            "input {} xkb_switch_layout {}",
                            input,
                            layouts
                                .get_key_value(&event.container.id)
                                .unwrap_or((&0, &0))
                                .1
                        ))
                        .await?;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
    unreachable!();
}
