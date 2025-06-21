use std::fmt::Display;

pub struct Node<T> {
    pub val: T,
    pub children: Vec<Node<T>>,
}

impl<T: Display> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            children: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: Node<T>) {
        self.children.push(val);
    }

    pub fn display(&self) {
        self.print_tree(0);
    }

    fn print_tree(&self, depth: usize) {
        print!("{}", "  ".repeat(depth));
        println!("{}", self.val);

        for child in &self.children {
            child.print_tree(depth + 1);
        }
    }
}
