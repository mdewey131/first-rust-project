use std::mem;

// Vectors are like arrays, but they're resizable

pub fn run(){
    // Arrays are fixed lists where elements are the same data types
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Modify a value within the vector
    numbers[2] = 20;
    println!("Second Position {}", numbers[2]);

    // Add a value onto the vector
    numbers.push(5);
    numbers.push(6);
    println!("Afer push: {:?}", numbers);

    // "Pop" a number out of the vector. Like Python, pop the last value
    numbers.pop();
    println!("Afer pop: {:?}", numbers);

    // Get the length of a vector
    println!("Vector Length {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices from a vector
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number {}", x);

    };

    // Loop and mutate
    for x in numbers.iter_mut(){
        // Multiply by two
        *x *= 2;
    };
    println!("Numbers Vec: {:?}", numbers);
}