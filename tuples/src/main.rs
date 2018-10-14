fn main() {
    // Tuples are fixed length, but can contain different types in each field
    let tup = (1, 'c', 100000000);
    println!("1:{}, 2:{}, 3:{}", tup.0, tup.1, tup.2);

    // Set specific type for each tuple
    let tup2: (i8, f64, char) = (127, 3.14, 'A');
    println!("1:{}, 2:{}, 3:{}", tup2.0, tup2.1, tup2.2);

    // Destructure a tuple by placing each field into variables
    let tup3: (i8, f64, char) = (127, 3.14, 'A');
    let (x, y, z) = tup3;
    println!("1:{}, 2:{}, 3:{}", x, y, z);

    let tup4 = (127, 3.14, 'A');
    let (b, n, m) = tup4;
    println!("1:{}, 2:{}, 3:{}", b, n, m);
}
