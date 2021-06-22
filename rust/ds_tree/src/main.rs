mod tree;

fn main() {
    let mut test = tree::Node {
        data: String::from("hi")
    };

    test.data.push('!');
    println!("Data: {}", test.data);
}
