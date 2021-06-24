// Tree data structure implementation for Rust
use std::collections::HashMap;  // For structs

/* 
 * It will use a hashmap which will be
 * created after initialization
 */

 #[derive(Debug)] // printing debug information
pub struct Node<'a> {
    // Structure representing a node
    pub id: String,
    pub relations: HashMap<
        &'a str,
        RelationValue<'a>
    >,
}

impl<'a> Node<'a> {
    // Static method to create a new node
    pub fn new(id: String) -> Node<'a> {
        let mut tmp_node = Node {
            relations: HashMap::new(),
            id: String::from(id),
        };

        tmp_node.relations.insert("owner", RelationValue::Single(None));
        tmp_node.relations.insert("children", RelationValue::Vector(vec![])); // change to vector

        tmp_node
    }

    // Add a child to this node
    pub fn add_child(&'a self, child: &mut Node<'a>) {
        // Set the child's parent to this node
        *child.relations.get_mut("owner").unwrap() = RelationValue::Single(Some(self));
    }
}

#[derive(Debug)] // printing owner and children debug
pub enum RelationValue<'a> {
    Single(
        Option<&'a Node<'a>>
    ),
    Vector(
        Vec<&'a Node<'a>>
    )
}