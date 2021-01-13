pub fn run() {
    // infinite loop
    let mut count = 0;
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // while loop
    let mut count2 = 0;
    while count2 <= 100 {
        if count2 % 15 == 0 {
            println!("FizzBuzz");
        } else if count2 % 3 == 0 {
            println!("Fizz");
        } else if count2 % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count2);
        }

        count2 += 1;
    }

    // for range
    for x in 0..100 {
        println!("{}", x);
    }
}