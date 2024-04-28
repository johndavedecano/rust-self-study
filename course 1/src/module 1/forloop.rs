fn main() {
    for i in 0..100 {
        let mut str: String = "".to_string();
        if i % 3 == 0 {
            str = "Hello".to_string();
        }
        if i % 5 == 0 {
            str = format!(
                "{}{}{}",
                str,
                if str.is_empty() { "" } else { " " },
                "World"
            );
        }

        if str.is_empty() == false {
            println!("{}", str);
        }
    }
}
