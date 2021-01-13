use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // reassign the value
    numbers[2] = 43;

    println!("{:?}", numbers);

    // getting a single value
    println!("First value: {}", numbers[0]);

    // get array lenght
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array size: {} bytes", mem::size_of_val(&numbers));

    // get slice from 0 to 2 both inclusive
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}