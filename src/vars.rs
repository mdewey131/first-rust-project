pub fn run() {
    // Variables hold primitive data or references to data
    // Variables are immutable by default
    // Rust is block-scoped 
    let name = "Brad";
    let mut age = 37;
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define a constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at once
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}