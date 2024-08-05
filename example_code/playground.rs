use std::ops;

// one's functionality is defined by an interface (a trait in Rust)
trait Expand {
    fn expand(&self, x: u64, y: u64);
}

trait Transform {
    // fn animal_type(&self) -> &str;
    fn call(&self) -> &str;
}

struct Transf {
    x: f64,
    y: f64,
}

impl Transf {
    fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl ops::Shr<Transf> for Transf {
    type Output = Transf;

    fn shr(self, _rhs: Transf) -> Transf {
        println!("> Foo.add(Bar) was called with {} {}", self.x, self.y);


        Transf { x: 0.0, y: 0.0 }
    }
}

impl<T> Expand for T where T: Transform {
    fn expand(&self, x: u64, y: u64) {
        println!("The animal said {} - {} {}", self.call(), x, y);
    }
}

struct Add {}
struct Subtract {}

impl ops::Shr<Subtract> for Subtract {
    type Output = Subtract;

    fn shr(self, _rhs: Subtract) -> Subtract {
        println!("> Foo.add(Bar) was called");

        Subtract {}
    }
}

impl Transform for Add {
    // fn animal_type(&self) -> &str {
    //     "dog"
    // }

    fn call(&self) -> &str {
        "woof"
    }
}

impl Transform for Subtract {
    // fn animal_type(&self) -> &str {
    //     "cat"
    // }

    fn call(&self) -> &str {
        "meow"
    }
}

fn main() {
    let add = Add {};
    let subtract = Subtract {};
    add.expand(1, 2);
    subtract.expand(1, 2);
}