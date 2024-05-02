struct Person {
    name: String,
}

struct Dog<'l> {
    name: String,
    owner: &'l Person,
}

fn main() {
    let p1 = Person {
        name: String::from("John"),
    };
    let p2 = Dog {
        name: String::from("Max"),
        owner: &p1,
    };
}
