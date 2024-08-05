In Rust, `struct`, `trait`, and `impl` are key language constructs used to define data and behavior.

### Struct
A `struct` is a composite data type that groups together zero or more other values with potentially different types into one type. It's similar to `class` in other languages. Here's an example:

```rust
struct Point {
    x: f64,
    y: f64,
}
```

This defines a `Point` struct with two fields, `x` and `y`.

### Trait
A `trait` is a way to define shared behavior across types. You can think of it as similar to an interface in other languages. A trait can be implemented by any type. Here's an example:

```rust
trait Drawable {
    fn draw(&self);
}
```

This defines a `Drawable` trait with one method, `draw`. 

### Impl

The `impl` keyword is used to define implementations on types. This includes implementing methods on structs, and implementing traits for types. Here's an example:

```rust
impl Point {
    fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Drawable for Point {
    fn draw(&self) {
        println!("Drawing point at ({}, {})", self.x, self.y);
    }
}
```

The first `impl` block defines a method `distance_to_origin` on the `Point` struct. The second `impl` block implements the `Drawable` trait for `Point`.