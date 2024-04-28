#[allow(unused_variables, unused_assignments, unused_mut)]

fn main() {
    // for i in 1..10 {
    //     say_hi();
    // }

    let mut name = "John";

    pass_by_ref(&mut name);

    println!("{}", name);

    // say_hello(name);
}

fn pass_by_ref(name: &mut &str) {
    println!("{}", name);
    *name = "Alex";
}

// fn say_hello(name: &str) {
//     println!("hello {}", name);
// }

// fn say_hi() {
//     println!("hello world!");
// }
