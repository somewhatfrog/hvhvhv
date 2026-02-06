use swayipc::{Connection, Event, EventType, Fallible, Node, NodeLayout, Rect, WindowChange};

fn find_focused_and_parent_layout(node: &Node, parent_layout: NodeLayout) -> Option<(Rect, NodeLayout)> {
    if node.focused {
        return Some((node.rect, parent_layout));
    }
    node.nodes.iter().find_map(|child| find_focused_and_parent_layout(child, node.layout))
}

fn set_layout(conn: &mut Connection) -> Fallible<()> {
    let tree = conn.get_tree()?;
    if let Some((rect, parent_layout)) = find_focused_and_parent_layout(&tree, tree.layout) {
        match (rect.height > rect.width, parent_layout) {
            (_, NodeLayout::Tabbed | NodeLayout::Stacked) => {}
            (true, NodeLayout::SplitH) => { conn.run_command("split v")?; }
            (false, NodeLayout::SplitV) => { conn.run_command("split h")?; }
            _ => {}
        }
    }
    Ok(())
}

fn main() -> Fallible<()> {
    // Create connection and sub to events, subscribe() consumes the connection
    let conn = Connection::new()?;
    let events = conn.subscribe(&[EventType::Window])?;
    // New connection for sending commands
    let mut cmd_conn = Connection::new()?;
    for event in events {
        match event? {
            Event::Window(window_event) if window_event.change == WindowChange::Focus => {
                if let Err(e) = set_layout(&mut cmd_conn) {
                    eprintln!("Error setting layout: {}", e);
                }
            }
            _ => {}
        }
    }
    Ok(())
}
