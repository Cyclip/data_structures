// Tree data structure implementation for Rust

pub struct Node<T> {
    // Structure representing a node and it's children
    pub data: T
}

impl<T> Node<T> {
    // Static method to create a new instance of Node
    pub fn new(data: T) -> Node<T> {
        let mut new = Node {data};
        return new;
    }
}
