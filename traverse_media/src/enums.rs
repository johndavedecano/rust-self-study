// enums are types which have a few definite values

enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // perform action depending on info
    match m {
        Movement::Up => println!("Avatar Moving Up"),
        Movement::Down => println!("Avatar Moving Down"),
        Movement::Left => println!("Avatar Moving Left"),
        Movement::Right => println!("Avatar Moving Right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}