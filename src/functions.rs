pub fn run() {
    // Functions - Used to store blocks of code for re-use
    greeting("Hello", "Jane");
    
    // Bind function values to variables
    let get_sum = add(32, 64);
    println!("Sum: {}", get_sum);

    // Closure
    // These are neat because they allow you to escape scope
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 3)); 
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// Arrow tells Rust what type is expected to be returned
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

