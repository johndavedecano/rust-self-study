use std::{
    sync::{Arc, Mutex},
    thread,
};

// mutual execution lock
// only one thread can access the data at any one time
fn main() {
    let c = Arc::new(Mutex::new(0));

    let mut threads = vec![];

    for i in 0..10 {
        let c = Arc::clone(&c);
        let thread = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        threads.push(thread);
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("Result {}", *c.lock().unwrap());
}
