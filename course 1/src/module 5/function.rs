fn main() {
    let mut name = "John";
    let greeting = say_hello(&mut name);
    println!("{}", greeting);
}

fn say_hello(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    greeting
}
