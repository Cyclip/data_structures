mod tree;
use std::io::{self, BufRead};

fn main() {
    let mut tree = tree::Tree::new();
    let node1 = tree::Node::new(String::from("testnode"), 5u32);
    let node2 = tree::Node::new(String::from("testnode2"), 5u32);
    let node3 = tree::Node::new(String::from("testnode3"), 5u32);

    // Add nodes
    tree.add_node(node1);
    tree.add_node(node2);
    tree.add_node(node3);    
    
    let look_for_id = input();

    let index = match tree.node_index(look_for_id.clone()) {
        Some(n) => n.to_string(),
        None => String::from("failed"),
    };

   // println!("Tree: {:#?}", tree);
    println!("Node matching {}: {:?}", look_for_id, index);
}

fn input() -> String {
    let mut str = String::new();
    let stdin = io::stdin();

    stdin
        .lock()
        .read_line(&mut str)
        .unwrap();
    
    io::Write::flush(&mut io::stdout());

    trim_newline(&mut str)
}

fn trim_newline(s: &mut String) -> String {
    if s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    return s.to_string();
}