fn main() {
    // let _primes: Vec<i32> = Vec::new();
    let mut primes = vec![1, 2, 3, 4, 5];

    primes.push(9);

    // primes[6] = 10;

    println!("{:?}", primes);

    for number in primes.iter() {
        println!("{}", number * primes.len());
    }
}
