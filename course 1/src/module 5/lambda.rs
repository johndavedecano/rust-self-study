// A functin within a function
// lambda function
// can be assigned to a variable
fn main() {
    let sum = |a: i32, b: i32| -> i32 { a + b };
    let min = |a: i32, b: i32| -> i32 { a - b };
    let div = |a: i32, b: i32| -> i32 { a / b };

    println!("{} + {} = {}", 25, 25, calculate(25, 25, sum));
    println!("{} - {} = {}", 100, 25, calculate(100, 25, min));
    println!("{} / {} = {}", 100, 25, calculate(100, 25, div));
}

fn calculate(a: i32, b: i32, c: fn(i32, i32) -> i32) -> i32 {
    c(a, b)
}
