trait Animal {
    fn make_noice(&self) -> &'static str;
}

struct Dog {}
struct Cat {}

impl Animal for Dog {
    fn make_noice(&self) -> &'static str {
        "bark"
    }
}
impl Animal for Cat {
    fn make_noice(&self) -> &'static str {
        "meow"
    }
}

fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        return Box::new(Dog {});
    } else {
        return Box::new(Cat {});
    }
}

fn main() {
    println!("The animal says {}", get_animal(1.0).make_noice());
    println!("The animal says {}", get_animal(0.5).make_noice());
}
