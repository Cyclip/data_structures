// Tree data structure implementation for Rust

// A singular node
#[derive(Debug)]
pub struct Node<T> {
    pub id: String,
    pub val: T,
}

impl<T> Node<T> {
    // Create a new node
    pub fn new(id: String, val: T) -> Node<T> {
        return Node {
            id,
            val
        };
    }
}

// Tree containing and managing all nodes
#[derive(Debug)]
pub struct Tree<T> {
    pub nodes: Vec<Node<T>>,
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
          self.nodes.push(node);
      }

      // Get node
      pub fn get_node(&self, id: String) -> Option<&Node<T>> {
          match self.nodes.iter().position(|x| x.id == id) {
                Some(n) => {
                    // Found node
                    Some(&self.nodes[n])
                },
                None => {
                    // Couldn't find node
                    None
                }
            }
      }
}