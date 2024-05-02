use std::rc::Rc;

struct Car {
    brand: Rc<String>,
}

impl Car {
    fn new(brand: Rc<String>) -> Car {
        Car { brand: brand }
    }
    fn drive(&self) {
        println!("{}", &self.brand);
    }
}

fn main() {
    let brand = Rc::new(String::from("BMW"));
    println!("pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("pointers: {}", Rc::strong_count(&brand));
    }
    println!("My car is {}", brand);
    println!("pointers: {}", Rc::strong_count(&brand));
}
