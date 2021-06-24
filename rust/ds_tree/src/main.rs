mod tree;

fn main() {
    let mut test = tree::Node::new(String::from("testnode"));
    let mut test2 = tree::Node::new(String::from("testnode2"));
    
    // Make test2 a child of test
    // test.add_child(&mut test2);
}
