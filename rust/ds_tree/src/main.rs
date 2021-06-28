mod tree;

fn main() {
    let mut tree = tree::Tree::new();
    let node1 = tree::Node::new(String::from("testnode"), 5u32);
    let node2 = tree::Node::new(String::from("testnode2"), 5u32);
    let node3 = tree::Node::new(String::from("testnode3"), 5u32);

    let look_for_id = String::from("testnode2");

    tree.add_node(node1);
    println!("Tree: {:#?}", tree);
    println!("Node: {:?}", tree.get_node(look_for_id));
}
