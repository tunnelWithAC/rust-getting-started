use std::ops;

use std::any::Any;
use std::fmt;

pub struct Transform {
    // children: Vec<Box<dyn Any>>,
    // children: Vec<Box<Transform>>,
    children: Vec<Transform>,
    // label: String,
}

impl ops::Shr<Transform> for Transform {
    type Output = Transform;

    fn shr(mut self, _rhs: Transform) -> Transform {
        // println!("> Foo.add(Bar) was called with {} {}", self.x, self.y);

        self.children.push(_rhs);
        self
        // Transform { x: 0.0, y: 0.0, my_array: [1], }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_with_no_children_length() {
        // let transform = Transform { children: vec![], label: "test".parse().unwrap() };
        let transform = Transform { children: vec![] };

        assert_eq!(transform.children.len(), 0);
    }

    #[test]
    fn test_transform_add_children() {
        // let transform = Transform { children: vec![], label: "test".parse().unwrap() };
        let transform = Transform { children: vec![] };
        let transform_two = Transform { children: vec![] };

        transform >> transform_two;

        assert_eq!(transform.children.len(), 1);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
// }



// impl Transform {
//     pub fn new() -> Self {
//         Self {
//             children: Vec::new(),
//             label: String::from("Transform"),
//         }
//     }
//
//     // pub fn set_label(&mut self, label: &str) -> &mut Self {
//     //     self.label = String::from(label);
//     //     self
//     // }
//
//     // pub fn add_child(&mut self, child: Box<dyn Any>) -> &mut Self {
//     //     self.children.push(child);
//     //     self
//     // }
//
//     // You'll need to define what the call method does in Rust
//     // pub fn call(&self, input: Option<&str>) {
//     //     unimplemented!()
//     // }
//
//     // You'll need to define what the expand method does in Rust
//     // pub fn expand(&self, input: Option<&str>) {
//     //     unimplemented!()
//     // }
// }

// impl fmt::Display for Transform {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Transform -> ({})", self.children.iter().map(|_| "child").collect::<Vec<&str>>().join(", "))
//     }
// }