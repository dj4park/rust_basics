pub fn run() {
    // i32 by default
    let x = 1;

    // f64 by default
    let y = 2.5;

    // explicit type definition
    let z: i64 = 95684645684;
    
    // max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active: bool = true;

    // boolean from expression
    let is_greater = 10 > 4;

    // character
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}