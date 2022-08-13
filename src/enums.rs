// Enums are types which have a few definite values
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action depending on info 
    match m {
        Movement::Up => println!("Character Moving Up"),
        Movement::Down => println!("Character Moving Down"),
        Movement::Left => println!("Character Moving Left"),
        Movement::Right => println!("Character Moving Right")
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Up;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}
