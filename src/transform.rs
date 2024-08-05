use std::any::Any;
use std::fmt;

pub struct Transform {
    children: Vec<Box<dyn Any>>,
    label: String,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            label: String::from("Transform"),
        }
    }

    // pub fn set_label(&mut self, label: &str) -> &mut Self {
    //     self.label = String::from(label);
    //     self
    // }

    // pub fn add_child(&mut self, child: Box<dyn Any>) -> &mut Self {
    //     self.children.push(child);
    //     self
    // }

    // You'll need to define what the call method does in Rust
    // pub fn call(&self, input: Option<&str>) {
    //     unimplemented!()
    // }

    // You'll need to define what the expand method does in Rust
    // pub fn expand(&self, input: Option<&str>) {
    //     unimplemented!()
    // }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transform -> ({})", self.children.iter().map(|_| "child").collect::<Vec<&str>>().join(", "))
    }
}