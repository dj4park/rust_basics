pub fn run() {
    greetings("Hello", "Alex");

    let get_sum = add(3, 3);
    println!("Sum: {}", get_sum);

    // closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(1, 2));
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet you.", greet, name);
}

// without semicolon it returns
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}