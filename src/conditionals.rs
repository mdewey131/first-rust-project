// Used to check the condition of something and act on the result
pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // If/else
    if age >= 21 && (check_id || knows_person_of_age) {
        println!("Bartender says 'What would you like to drink?'");
    } else if age < 21 && check_id {
        println!("Bartender says 'First off, how dare you?'");
    } else {
        println!("Bartender asks you for your ID");
    }

    // Shorthand if
    let is_of_age: bool = if age >= 21 {true} else {false};
    println!("Is of age? {}", is_of_age);

    
}