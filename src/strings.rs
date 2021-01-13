pub fn run() {
    let mut hello = String::from("Hello ");

    // get length of str
    let length = hello.len();

    // push char
    hello.push('W');

    // push a string
    hello.push_str("orld!");

    // assertion testing
    assert_eq!(2, length);

    println!("{:?}", ((hello, length)));
}