pub fn run() {
    // There are many types of loop
 
    // Infinite loop
    let mut count: i32 = 0;
    loop {
        count += 1;
        println!("Number {}", count);

        if count == 20 {
            break;
        }
    }

    // While loop (FizzBuzz)
    // let mut fb_count: u8 = 0;
    // while fb_count < 100 {
    //     fb_count += 1;
    //     if fb_count % 5 == 0 && fb_count % 3 == 0 {
    //         println!("FizzBuzz");
    //     } else if fb_count % 3 == 0 {
    //         println!("Fizz");
    //     } else if fb_count % 5 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("{}", fb_count);
    //     }
    // }

        // Similar idea but with a loop and a generator
    for x in 0..100{
        if x % 5 == 0 && x % 3 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }

}