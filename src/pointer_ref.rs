pub fn run() {
    // primitive array
    let a1 = [1, 2, 3];
    let a2 = a1;

    println!("Values: {:?}", (a1, a2));

    // non-primitive
    // vector
    let v1 = vec![1, 2, 3];

    // this causes error
    // let v2 = v1;
    let v2 = &v1;
    println!("Vectors: {:?}", (&v1, v2));

}