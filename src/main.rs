mod transform;

// fn main() {
//     println!("Hello, world!");
// }

use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    value: T,
    children: Vec<Box<Node<T>>>,
}

fn main() {
    let root = Node {
        value: 10,
        children: vec![
            Box::new(Node {
                value: 20,
                children: vec![
                    Box::new(Node {
                        value: 30,
                        children: vec![],
                    }),
                    Box::new(Node {
                        value: 40,
                        children: vec![],
                    }),
                ],
            }),
            Box::new(Node {
                value: 50,
                children: vec![],
            }),
        ],
    };

    println!("{:?}", root);
}
