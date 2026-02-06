use swayipc::{Connection, Event, EventType, Fallible, Node, NodeLayout, Rect, WindowChange};

fn find_focused_and_parent_layout(node: &Node, parent_layout: Option<NodeLayout>) -> Option<(Rect, NodeLayout)> {
    if node.focused {
        if let Some(layout) = parent_layout {
            return Some((node.rect, layout));
        }
    }
    for child in &node.nodes {
        if let Some(result) = find_focused_and_parent_layout(child, Some(node.layout)) {
            return Some(result);
        }
    }
    None
}

fn set_layout(conn: &mut Connection) -> Fallible<()> {
    let tree = conn.get_tree()?;
    if let Some((rect, parent_layout)) = find_focused_and_parent_layout(&tree, None) {
        if matches!(parent_layout, NodeLayout::Tabbed | NodeLayout::Stacked) {
            return Ok(());
        }
        if rect.height > rect.width {
            if parent_layout == NodeLayout::SplitH {
                conn.run_command("split v")?;
            }
        } else {
            if parent_layout == NodeLayout::SplitV {
                conn.run_command("split h")?;
            }
        }
    }
    Ok(())
}

fn main() -> Fallible<()> {
    // Create connection and sub to events, subscribe() consumes the connection
    let conn = Connection::new()?;
    let events = conn.subscribe(&[EventType::Window])?;

    // New connection for sending commands
    for event in events {
        match event? {
            Event::Window(window_event) => {
                if window_event.change == WindowChange::Focus {
                    match Connection::new() {
                        Ok(mut cmd_conn) => {
                            if let Err(e) = set_layout(&mut cmd_conn) {
                                eprintln!("Error setting layout: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error creating command connection: {}", e);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
