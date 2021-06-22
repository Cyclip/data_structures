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
    pub fn add_child(&self, mut child: Node<T>) {
        // Only add if the new child instance isn't already a child
        if !self.has_child(child) {
            // Set relations
            child.parent = Some(Box::new(*self));
            self.children.push(Box::new(child));
        }
    }

    // Method to remove a child from this Node instance
    pub fn remove_child(&self, child: Node<T>) {
        // Remove their parent identifier
        child.parent = None;

        // Iterate and remove the child instance
        self.children.remove(
            self.children.iter().position(
                /* Closure taking a pointer to the element
                 * and checking if it matches a condition.
                */
                |&x| self.compare_nodes(*x, child)
            ).unwrap()
        );
    }

    pub fn has_child(&self, child: Node<T>) -> bool {
        false
    }

    fn compare_nodes(&self, a: Node<T>, b: Node<T>) -> bool {
        &a as *const _ == &b as *const _
    }
}
