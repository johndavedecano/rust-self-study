#[allow(unused_variables, unused_assignments)]

fn main() {
    let name = "Alex";
    let mut age = 32;
    let amount: i64 = 85858585858585858;

    // Names can contain letters, numbers and underscores
    // Must start with a letter or underscore
    // follow snake_case naming convention
    // Immutable by default
    let length = 34;
    // length = 35; // error
    // can be declared as mutable
    // immutable by default
    age = 32;
    // shadowing is allowed
    let color = "blue";
    let color = 86;
    // can declare variables simultaneously
    let (a, b, c) = (43, 85, "red");
}
