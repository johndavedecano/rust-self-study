pub fn run() {
    // Print to console
    println!("Hello from print rs file");
    println!("Number: {}", 1);
    // basic formatting
    println!("{} is from {}", "Brad", "Mass");
    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");
    // named arguments
    println!(
    "{name} is good",
    name = "dave"
    );
    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);


    // placeholder for debugging traits
    println!("{:?}", (12, true, "hello"));

    // placeholder for math exp
    println!("10 + 10 = {}", (10 + 10));
}