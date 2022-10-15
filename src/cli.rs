use std::env;

// command line arguments
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Brad";
    let status = "100%";

    // println!("Command: {:?}", command);

    if command == "hello" {
        println!("{} {}, how are you?",command, name);
    } else if command == "status" {
        println!("the status is {}", status);
    } else {
        println!("sorry but command was not found.");
    }
}