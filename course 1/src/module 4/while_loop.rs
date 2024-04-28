fn main() {
    let mut count = 0;

    while true {
        if count == 100 {
            break;
        }
        count += 1;
        println!("{}", count);
    }
}
