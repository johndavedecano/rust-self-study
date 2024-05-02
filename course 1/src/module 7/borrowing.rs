fn main() {
    // only one variable can own a piece of memory
    // variables can borrow ownership
    let mut a = 6;
    let b = &mut a;

    println!("{}", *b);
    println!("{}", a);
}
