pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    numbers[2] = 20;

    println!("{:?}", numbers);

    // get single value from an array
    println!("Single Value: {}", numbers[0]);

    // arrays are stack allocated
    println!("This array occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);
}