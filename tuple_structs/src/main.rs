fn main() {
    // Tuple structs behave similarly to tuples, however they are created with their own types

    struct Color (i32,i32,i32);
    struct Point (i32,i32,i32);

    // These are both different types
    let black = Color(0,0,0);
    let point = Point(0,0,0);
    println!("colour: {} {} {}", black.0, black.1, black.2);
    println!("colour: {} {} {}", point.0, point.1, point.2);
}
