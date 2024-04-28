// Primitive str = Immutable fixed length string somewhere in memory
// Striong = Growable, heep allocated data structure

pub fn run() {

    let mut hello = String::from("Hello");

    // get length
    println!("Length:{}", hello.len());

    // push a char
    // hello.push('W');

    // push a string
    hello.push_str(" World");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if is empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains World {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(1, s.capacity());

    println!("{}", s);

    println!("{}", hello);
}