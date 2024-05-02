// A functin within a function
// lambda function
// can be assigned to a variable
fn main() {
    let square = |a| a * a;
    apply(square, 6);
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("{}", f(a));
}
