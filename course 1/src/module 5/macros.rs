// write code that writes code - meta programming
// match an expression and perform some operation
// code is expanded and compiled

macro_rules! my_macro {
    () => {
        println!("First macro")
    };
}

macro_rules! name {
    ($name: expr) => {
        println!("Hey {}", $name)
    };
}

fn main() {
    my_macro!();
    name!("test");
}
