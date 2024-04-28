fn main() {
    print_choice(Suit::Heart);
    print_choice(Suit::Club);
    print_choice(Suit::Spade);
    print_choice(Suit::Diamond);

    country(44);
    country(34);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "invalid",
    };
    println!("country is {}", country);
}

fn print_choice(choice: Suit) {
    match choice {
        Suit::Heart => {
            println!("\u{2665}")
        }
        Suit::Spade => {
            println!("\u{2660}")
        }
        Suit::Club => {
            println!("\u{2663}")
        }
        Suit::Diamond => {
            println!("\u{2666}")
        }
    }
}
