use std::mem;

pub fn run(){
    // Arrays are fixed lists where elements are the same data types
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Modify a value within the array
    numbers[2] = 20;
    println!("Second Position {}", numbers[2]);

    // Get the length of an array
    println!("Array Length {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices from an array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}