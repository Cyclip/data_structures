mod tree;

fn main() {
    let mut test = tree::Node::new(Strimg::from("testnode"));
    println!("{:?}", test.relations.get("owner"));
}
