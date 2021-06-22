// Tree data structure implementation for Rust

#[derive(PartialEq, PartialOrd)]
pub struct Node<T> {
    /*
     * Structure representing a node and its children/parents
     * It's recommended to not directly change these variables,
     * but use the methods implemented in the struct.
     *
     * Derives PartialEq & PartialOrd for comparing
     */
    pub data: T,
    pub parent: Option<Box<Node<T>>>,
    pub children: Vec<Box<Node<T>>>,

    root: bool,
}

impl<T> Node<T> {
    // Static method to create a new instance of Node
    pub fn new(data: T) -> Node<T> {
        let mut new = Node {
            data,
            parent: None,
            children: Vec::new(),
            root: false,
        };

        return new;
    }

    // Method to add a child to this Node instance
    pub fn add_child(&self, child: Node<T>) {
        // Only add if the new child instance isn't already a child
        if !self.has_child(child) {
            // Set relations
            child.parent = Some(Box::new(*self));
            self.children.push(Box::new(child));
        }
    }
}
