// Very important in Rust, apparently

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct ColorTup(u8, u8, u8);

// A Struct With Functions
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(), 
            last_name: last.to_string()
        }
    }

    // Get the full name of a person
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    // Set a new last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    let c_tup = ColorTup(255, 0, 0);

    c.red = 200;

    println!("Color Normal: {} {} {}", c.red, c.green, c.blue);
    println!("Color Tup: {} {} {}", c_tup.0, c_tup.1, c_tup.2);
    
    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person with full name: {}", p.full_name());
    p.set_last_name("Williams");
    println!("After name change {}", p.full_name());
    println!("To Tuple {:?}", p.to_tuple());

}