trait Speaks {
    fn speak(&self);
}

trait Animal {
    // fn animal_type(&self) -> &str;
    fn noise(&self) -> &str;
}

impl<T> Speaks for T where T: Animal {
    fn speak(&self) {
        println!("The animal said {}", self.noise());
    }
}

struct Dog {}
struct Cat {}

impl Animal for Dog {
    // fn animal_type(&self) -> &str {
    //     "dog"
    // }

    fn noise(&self) -> &str {
        "woof"
    }
}

impl Animal for Cat {
    // fn animal_type(&self) -> &str {
    //     "cat"
    // }

    fn noise(&self) -> &str {
        "meow"
    }
}

fn main() {
    let dog = Dog {};
    let cat = Cat {};
    dog.speak();
    cat.speak();
}