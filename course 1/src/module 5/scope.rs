fn main() {
    let a = 2;
    {
        let a = a + 5;
        println!("{}", a);
    }
    println!("{}", a);
}
