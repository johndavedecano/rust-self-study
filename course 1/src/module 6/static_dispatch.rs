trait Duplicatable {
    fn dup(&self) -> String;
}

impl Duplicatable for String {
    fn dup(&self) -> String {
        format!("{0}-{0}", *self)
    }
}

impl Duplicatable for i32 {
    fn dup(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplicate<T: Duplicatable>(x: T) {
    println!("{}", x.dup());
}

fn main() {
    let a = 42;
    let b = "Hi John".to_string();
    duplicate(a);
    duplicate(b);
}
