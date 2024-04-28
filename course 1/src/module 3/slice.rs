fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &[1..2];

    println!("{:?}", slice.len());

    for x in numbers.iter() {
        println!("{:?}", x);
    }
}
