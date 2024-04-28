fn main() {
    // arrays: collection of values of the same type
    let prime = [2, 4, 6, 5, 4, 3, 4, 5, 6];
    let doubles = [1.0, 1.2, 1.3, 1.4];

    println!("{:?}", prime);
    println!("{:?}", doubles);

    for number in doubles.iter() {
        println!("{}", number);
    }

    for number in prime.iter() {
        println!("{}", number);
    }
}
