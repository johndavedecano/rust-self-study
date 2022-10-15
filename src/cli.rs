use std::env;

// command line arguments
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("Command: {:?}", command);
}