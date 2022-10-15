// variables hold primitive data or reference to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} i am {}", name, age);
    age = 39;
    println!("My name is {} i am {}", name, age);

    const ID: i32 = 001;

    println!("ID: {}", ID);

    let (my_name, my_age) = ("Brad", 37);

    println!("{} is {}", my_name, my_age);
}