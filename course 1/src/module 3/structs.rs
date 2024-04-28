fn main() {
    let emp1 = Employee {
        name: String::from("hello"),
        company: String::from("google"),
        age: 35,
    };
    println!("{:?}", emp1);

    println!("{}", emp1.name);

    println!("{}", emp1.print_details());

    println!("{}", Employee::test("Dave"));
}

#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32,
}

impl Employee {
    fn print_details(&self) -> String {
        format!(
            "name: {}, age: {}, company: {}",
            self.name, self.age, self.company
        )
    }

    fn test(name: &str) -> String {
        format!("Hello {}!", name)
    }
}
