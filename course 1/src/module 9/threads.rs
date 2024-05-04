use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let mut threads = vec![];

    for i in 0..10 {
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(i * 1000));
            println!("new thread {}", i);
        });
        threads.push(th);
    }

    for t in threads {
        t.join();
    }

    println!("main thread")
}
