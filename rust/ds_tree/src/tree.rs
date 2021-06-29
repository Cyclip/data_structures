// Tree data structure implementation for Rust

// A singular node
#[derive(Debug)]
pub struct Node<T> {
    pub id: Identifier,
    pub val: T,
}

impl<T> Node<T> {
    // Create a new node
    pub fn new(id: String, val: T) -> Node<T> {
        return Node {
            id: Identifier::Name(id),
            val
        };
    }

    // Get ID
    pub fn get_id(&self) -> String {
        match &self.id {
            Identifier::Name(s) => s.to_string(),
            _ => {panic!("Can't find ID");}
        }
    }
}

// Tree containing and managing all nodes
#[derive(Debug)]
pub struct Tree<T> {
    pub nodes: Vec<NodeDetails<T>>,
}

impl<T> Tree<T> {
      // Create a new tree
      pub fn new() -> Tree<T> {
          return Tree {
              nodes: Vec::new(),
          };
      }

      // Register a node
      pub fn add_node(&mut self, node: Node<T>) {
          self.nodes.push(NodeDetails::new(node));
      }

      // Get node index
      pub fn node_index(&self, id: String) -> Option<usize> {
          match self.nodes.iter().position(|x| x.node.get_id() == id) {
                Some(n) => Some(n),
                None => None
            }
      }

      // Get node by index
      pub fn get_node(&self, index: usize) -> &Node<T> {
          let ref node_ref = self.nodes[index];
          &node_ref.node
      }

      // Set node's parent
      pub fn set_parent(&self, node: String, parent: String) {
          let ref mut child_node = match self.node_index(node) {
              Some(n) => &self.nodes[n],
              None => {panic!("Child node does not exist");},
          };

          let parent_node_details = match self.node_index(parent) {
              Some(n) => &self.nodes[n],
              None => {panic!("Parent node does not exist");},
          };

          let parent_node = Some(&parent_node_details.node);

          child_node.parent = *parent_node;
      }
}

// Node management
#[derive(Debug)]
pub struct NodeDetails<T> {
    node: Node<T>,
    parent: Option<Node<T>>,
}

impl<T> NodeDetails<T> {
    pub fn new(node: Node<T>) -> NodeDetails<T> {
        return NodeDetails {
            node,
            parent: None,
        }
    }

}

// Identifier for each node
#[derive(Debug)]
pub enum Identifier {
    Name(String),
    Index(usize),
}