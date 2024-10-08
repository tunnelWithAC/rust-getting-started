// mod transform;
use std::ops;
// use std::rc::Rc;
// use std::any::Any;
// use std::fmt;
use std::fmt::Debug;

struct Transform<'a> {
    // value: T,
    children: Vec<Box< Transform<'a>>>,
}

impl<'a> Transform<'a> {
    fn new() -> Transform<'a> {
        Transform {
            children: Vec::new(),
        }
    }

    fn add(&mut self, other: &'a Transform<'a>) {
        self.children.push(Box::new(other));
    }
}

// `fn(transform::Transform<'_>, transform::Transform<'_>) -> transform::Transform<'a>`
// `fn(&mut transform::Transform<'_>, &'a transform::Transform<'_>) -> &transform::Transform<'_>`

// expected signature `fn(&mut transform::Transform<'_>, &mut transform::Transform<'_>) -> transform::Transform<'_>`
// ___found signature `fn(&mut transform::Transform<'_>, &'a transform::Transform<'_>) -> &'a transform::Transform<'_>`

impl<'a> ops::Shr<Transform<'a>> for &mut Transform<'a> {
    type Output = &'a Transform<'a>;

    fn shr(mut self, mut other: Transform<'a>) -> &'a Transform<'a> {
        self.children.push(Box::new(other));
        other
        // other // Return a reference to the modified self
    }
}


//
// // impl ops::Shr<Transform> for Transform {
// //     type Output = ();
// //     // type Output = Transform;
// //
// //     fn shr(self, rhs: Transform) -> Transform {
// //         let mut new_children = self.children;
// //         new_children.push(rhs);
// //         Transform { children: new_children }
// //     }
// // }
// //
// // impl ops::Shr<Transform> for Transform {
// //     type Output = Transform;
// //
// //     fn shr(&mut self, rhs: Transform) -> Transform {
// //         self.children.push(rhs);
// //         *self // Return a reference to the modified self
// //     }
// // }
//
// // impl ops::Shr<Transform> for Transform {
// //     type Output = Transform;
// //
// //     fn shr(&mut self, _rhs: Transform) -> Transform {
// //         self.children.push(_rhs);
// //         _rhs
// //     }
// // }
//
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
        // let transform = Transform { children: vec![] };
        // let transform_two = Transform { children: vec![] };
        let mut transform = Transform { children: vec![] };
        let transform_two = Transform { children: vec![] };

        // transform.
        // transform.add(&transform_two);
        transform >> transform_two;

        // let mut my_struct = MyStruct { children: vec![] };
        // let added_value = my_struct.add(5);


        // transform >> transform_two;

        assert_eq!(transform.children.len(), 1);
    }
}
//
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //
// //     #[test]
// //     fn it_adds_two() {
// //         assert_eq!(4, add_two(2));
// //     }
// // }
//
//
//
// // impl Transform {
// //     pub fn new() -> Self {
// //         Self {
// //             children: Vec::new(),
// //             label: String::from("Transform"),
// //         }
// //     }
// //
// //     // pub fn set_label(&mut self, label: &str) -> &mut Self {
// //     //     self.label = String::from(label);
// //     //     self
// //     // }
// //
// //     // pub fn add_child(&mut self, child: Box<dyn Any>) -> &mut Self {
// //     //     self.children.push(child);
// //     //     self
// //     // }
// //
// //     // You'll need to define what the call method does in Rust
// //     // pub fn call(&self, input: Option<&str>) {
// //     //     unimplemented!()
// //     // }
// //
// //     // You'll need to define what the expand method does in Rust
// //     // pub fn expand(&self, input: Option<&str>) {
// //     //     unimplemented!()
// //     // }
// // }
//
// // impl fmt::Display for Transform {
// //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// //         write!(f, "Transform -> ({})", self.children.iter().map(|_| "child").collect::<Vec<&str>>().join(", "))
// //     }
// // }