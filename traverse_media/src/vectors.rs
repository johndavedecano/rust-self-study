// vectors are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers[2] = 20;

    // add on to vector
    numbers.push(5);
    numbers.push(6);

    // pop off the last value
    numbers.pop(); // 6 is gone

    println!("{:?}", numbers);

    // // get single value from an array
    println!("Single Value: {}", numbers[0]);

    // get vector length
    println!("Vector Length: {}", numbers.len());

    // // arrays are stack allocated
    println!("This vector  occupies {} bytes", std::mem::size_of_val(&numbers));

    // // get slice
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);

    // loop through a vector value
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop through and mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}