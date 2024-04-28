use crate::Colors::Red;

fn main() {
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    let my_color = Red;
    println!("{:?}", my_color);
}

#[derive(Debug)]
enum Colors {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Age(u32),
    Company(String),
}
