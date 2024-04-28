#[allow(unused_variables, unused_assignments)]

fn main() {
    // let cat: &str = "Fluffy";
    let cat: &'static str = "Fluffy";
    // string slices and string objects
    // let dog: String = String::new();
    // let dog: String = String::from("Max");
    // println!("{}", dog);

    // let owner = format!("Hi I am the the owner {} of {}", "mark", dog);

    // println!("{}", owner);
    let mut dog = String::new();

    dog.push(' ');

    println!("{}", dog.len());

    dog.push_str("hello world woh woh wof wof");

    println!("{}", dog);

    let new_dog = dog.replace("hello", "holla");

    println!("{}", new_dog);
}
