pub fn run() {
    let x = 1;


    let y = 2.5;

    // add explicit type
    let z: i64 = 454545454545454545;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Booleans are true or false
    let is_active = true;

    let is_greater = 10 < 5;

    let a1 = 'a';

    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}