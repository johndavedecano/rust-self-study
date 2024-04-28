use std::u32;

fn main() {
    let mut a: u32 = 0;
    loop {
        print!("{}\n", a);
        a += 1;
        if a >= 100 {
            break;
        }
    }
}
