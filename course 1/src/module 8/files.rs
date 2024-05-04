use std::{
    fs::{remove_file, File, OpenOptions},
    io::{Read, Write},
};

fn main() {
    let mut file = File::create("src/example.txt").expect("create failed");

    file.write_all("hello world".as_bytes())
        .expect("write failed");

    // let mut file = OpenOptions::new()
    //     .append(true)
    //     .open("src/example.txt")
    //     .expect("cannot open file");

    // file.write_all("\nAdding contents to the file".as_bytes())
    //     .expect("unable to write to the file");

    let mut file = std::fs::File::open("src/example.txt").unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);

    remove_file("src/example.txt").expect("delete file");
}
