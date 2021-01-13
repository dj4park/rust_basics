// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct Color2(u8, u8, u8);

// more complex struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set the last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c1 = Color {
        red: 255,
        green: 0, 
        blue: 0
    }; 
    c1.red = 200;
    println!("Color: {} {} {}", c1.red, c1.green, c1.blue);

    let mut c2 = Color2(255, 0, 0);
    c2.0 = 123;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Alex", "Park");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Full Name: {}", p.full_name());
    p.set_last_name("Kim");
    println!("Full Name: {}", p.full_name());
    println!("Name to tuple: {:?}", p.to_tuple());

}