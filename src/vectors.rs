use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // reassign the value
    numbers[2] = 43;

    // add to vector
    numbers.push(32);
    numbers.push(1);

    // pop off vector
    numbers.pop();

    println!("{:?}", numbers);

    // getting a single value
    println!("First value: {}", numbers[0]);

    // get vector lenght
    println!("Vector length: {}", numbers.len());

    // vectors are stack allocated
    println!("Vector size: {} bytes", mem::size_of_val(&numbers));

    // get slice from 0 to 2 both inclusive
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}