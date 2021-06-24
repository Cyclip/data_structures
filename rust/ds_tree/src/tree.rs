// Tree data structure implementation for Rust

#[derive(Debug)] // printing debug information
pub struct Node<'a> {
    // Structure representing a node
    pub id: String,
    pub parent: Option<&'a Node<'a>>,
    pub children: Vec<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    // Static method to create a new node
    pub fn new(id: String) -> Node<'a> {
        Node {
            id,
            parent: None,
            children: Vec::new(),
        }
    }

    // Add a child to this node
    pub fn add_child(&'a mut self, child: &'a mut Node<'a>) {
        self.children.push(child);
        child.parent = Some(self);
    }
}