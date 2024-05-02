struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("hello world")
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("hello using Rust")
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Java"
    }

    fn say_hello(&self) {
        println!("hello using Java")
    }
}

// Smilar to an interface or abstract class
//  Add a definition to a structure
// Can have definition only or default implementation
// Can have instance and non instance action
fn main() {
    let r = RustDev::new(true);
    let j = JavaDev::new(true);
    println!("{}", r.language());
    println!("{}", j.language());
    r.say_hello();
    j.say_hello();
}
