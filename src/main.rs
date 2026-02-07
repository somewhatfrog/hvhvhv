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
            (true, NodeLayout::SplitH) => { conn.run_command("splitv")?; }
            (false, NodeLayout::SplitV) => { conn.run_command("splith")?; }
            _ => {}
        }
    }
    Ok(())
}

fn main() -> Fallible<()> {
    let event_conn = Connection::new()?;
    let mut cmd_conn = Connection::new()?;
    for event in event_conn.subscribe(&[EventType::Window])? {
        match event? {
            Event::Window(window_event) if window_event.change == WindowChange::Focus => {
                if let Err(err) = set_layout(&mut cmd_conn) { eprintln!("Error: {}", err); }
            }
            _ => {}
        }
    }
    Ok(())
}
