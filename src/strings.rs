// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Using push on a char (i.e. one character)
    hello.push('W');

    // Push on an str
    hello.push_str("orld!");

    // Get the capacity in bytes 
    println!("Capacity: {}", hello.capacity());

    // Check empty
    println!("Is Empty {}", hello.is_empty());

    // Look to see if the string contains a substring
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through strings, here by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    println!("s is {}", s);
   // println!("{}", hello);

   // Testing assertions
   assert_eq!(2, s.len());
   assert_eq!(10, s.capacity());
}