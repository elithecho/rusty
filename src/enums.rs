
enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up    => println!("Avatar moving Up"),
        Movement::Down  => println!("Avatar moving Down"),
        Movement::Left  => println!("Avatar moving Left"),
        Movement::Right => println!("Avatar moving Right")
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
