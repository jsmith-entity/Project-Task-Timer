use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Node {
    pub heading: Option<String>,
    pub content: Vec<String>,
    pub children: Vec<Node>,

    pub total_time: Duration,
    pub content_times: Vec<Duration>,
    pub completed_tasks: Vec<bool>,
}

pub type NodePath = Vec<usize>;

impl Node {
    pub fn new() -> Self {
        Self {
            heading: None,
            content: Vec::new(),
            children: Vec::new(),

            total_time: Duration::default(),
            content_times: Vec::new(),
            completed_tasks: Vec::new(),
        }
    }

    pub fn new_with_heading(heading: String) -> Self {
        Self {
            heading: Some(heading),
            content: Vec::new(),
            children: Vec::new(),

            total_time: Duration::default(),
            content_times: Vec::new(),
            completed_tasks: Vec::new(),
        }
    }

    pub fn convert_from(buf: &str) -> Self {
        let lines: Vec<&str> = buf.split("\n").filter(|line| !line.is_empty()).collect();

        let mut root = Node::new();
        let mut stack: Vec<usize> = Vec::new();

        for line in lines {
            Node::convert_line(line, &mut root, &mut stack);
        }

        return root;
    }

    pub fn find_path(current: &Node, target: &Node, path: &mut NodePath) -> bool {
        if current == target {
            return true;
        }

        for (i, child) in current.children.iter().enumerate() {
            path.push(i);
            if Node::find_path(child, target, path) {
                return true;
            }

            path.pop();
        }

        return false;
    }

    pub fn get_node_mut(&mut self, path: &NodePath) -> Option<&mut Node> {
        let mut current = self;
        for &idx in path {
            current = current.children.get_mut(idx)?;
        }

        return Some(current);
    }

    pub fn get_node(&self, path: &NodePath) -> Option<&Node> {
        let mut current = self;
        for &idx in path {
            current = current.children.get(idx)?
        }

        return Some(current);
    }

    fn convert_line(line: &str, root: &mut Node, indices: &mut Vec<usize>) {
        let depth = line.chars().filter(|&c| c == '#').count();

        if depth > 0 {
            let heading = line.to_string();

            while indices.len() >= depth {
                indices.pop();
            }

            let parent_node = Node::find_heading_level(root, indices);
            parent_node.children.push(Node::new_with_heading(heading));

            // New heading level
            if depth > indices.len() {
                indices.push(parent_node.children.len() - 1);
            }
        } else {
            let content = line.to_string();

            let current_node = Node::find_heading_level(root, indices);
            current_node.content.push(content);
            current_node.content_times.push(Duration::from_secs(0));
            current_node.completed_tasks.push(false);
        }
    }

    fn find_heading_level<'a>(root: &'a mut Node, indices: &[usize]) -> &'a mut Node {
        let mut node = root;
        for &idx in indices {
            node = &mut node.children[idx];
        }
        return node;
    }
}
