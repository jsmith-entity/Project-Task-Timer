#[derive(Clone)]
pub struct Node {
    pub heading: Option<String>,
    pub content: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            heading: None,
            content: None,
            children: Vec::new(),
        }
    }

    pub fn new_with_heading(heading: String) -> Self {
        Self {
            heading: Some(heading),
            content: None,
            children: Vec::new(),
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
            match &mut current_node.content {
                Some(existing) => {
                    existing.push_str("\n");
                    existing.push_str(&content);
                }
                None => {
                    current_node.content = Some(content);
                }
            }
        }
    }

    fn find_heading_level<'a>(root: &'a mut Node, indices: &[usize]) -> &'a mut Node {
        let mut node = root;
        for &idx in indices {
            node = &mut node.children[idx];
        }
        return node;
    }

    pub fn print(&self, depth: usize) {
        let indent = "  ".repeat(depth);

        if let Some(heading) = &self.heading {
            println!("{}{}", indent, heading);
        }

        if let Some(content) = &self.content {
            for line in content.split("\n") {
                println!("{}{}", indent, line);
            }
        }

        for child_node in &self.children {
            child_node.print(depth + 1);
        }
    }
}
