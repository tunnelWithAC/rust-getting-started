In Rust, `struct` and `impl` are fundamental concepts.

1. `struct`: A `struct` is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you're coming from an object-oriented language, a `struct` is similar to an object's data attributes.

```rust
struct Point {
    x: i32,
    y: i32,
}
```

2. `impl`: `impl` is used to define methods on structs (or traits, enums). It's similar to defining methods inside a class in object-oriented languages. Inside an `impl` block, you can define methods that take a special first parameter, self, which represents the instance of the struct the method is being called on.

```
impl Point {
fn new(x: i32, y: i32) -> Self {
Self { x, y }
}

    fn distance(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}
```

In this example, `new` is an associated function (similar to a static method in other languages) and `distance` is a method that calculates the distance of the point from the origin.

* https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159/3