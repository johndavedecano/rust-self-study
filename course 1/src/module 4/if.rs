use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..10);

    if num >= 5 {
        println!("Number {} is greater or equal than 5", num);
    } else {
        println!("Number {} is smaller than 5", num);
    }

    if num > 5 {
        println!("{} > 5", num);
    } else if num == 5 {
        println!("{} == 5", num);
    } else {
        println!("{} < 5", num);
    }

    let res: bool = if num > 5 { true } else { false };

    println!("{} > 5", res);
}
