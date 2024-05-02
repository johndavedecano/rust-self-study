use std::fs::File;

fn main() {
    // unrecoverable error
    // le: Vec<i32>t v = vec![1, 2, 3];
    // v[10];
    // panic!("Something went wrong cannot proceed");

    // recoverable
    let f = File::open("main.jpg");

    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        }
        Err(e) => {
            println!("find not found {:?}", e);
        }
    }

    divide(Some(1));
    divide(Some(10));
    divide(None);
    // divide(Some(0));
}

const ANSWERE_TO_LIFE: i32 = 42;

fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("cannot be divided by zero"),
        Some(x) => println!("answer to life is {}", ANSWERE_TO_LIFE / x),
        None => println!("boooooo {}", ANSWERE_TO_LIFE),
    }
}
