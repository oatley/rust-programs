fn main() {
    let mut s = String::from("hello"); // This is a actual string (String)
    //let mut s = "hello"; // This is string literal or string slice(&str)
    s.push_str(", world"); // append to end of string
    println!("{}", s);
}
