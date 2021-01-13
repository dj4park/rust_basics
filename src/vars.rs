pub fn run() {
    let name = "Brad";

    // mutable variable
    let mut age = 37;
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // define a constant int32
    // need to add a type to a const var
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assing multiple variables
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}