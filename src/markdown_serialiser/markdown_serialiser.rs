use std::fs;

use crate::task_timer::node::Node;

pub fn export(root_node: Node, file_name: String) {
    let mut contents = String::new();
    for child_node in root_node.children.iter() {
        contents += &push_node(child_node, 0)
    }

    fs::write(file_name, contents).expect("erm");
}

fn push_node(node: &Node, indent_len: usize) -> String {
    let mut contents = String::new();

    if node.heading.is_some() {
        let heading = node.heading.clone().unwrap();
        contents.push_str(&format!("{}\n", heading));
    }

    for idx in 0..node.content.len() {
        let checkbox: String;
        if node.completed_tasks[idx] {
            checkbox = "- [x] ".to_string();
        } else {
            checkbox = "- [ ] ".to_string();
        }

        contents.push_str(&format!("{}{}\n", checkbox, node.content[idx]));
    }

    for child_node in node.children.iter() {
        contents += &push_node(child_node, indent_len + 1);
    }

    return contents;
}
