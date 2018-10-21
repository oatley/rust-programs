fn main() {
    let s1 = String::from("meow or is it mooo");
    let s2 = first_word_str(&s1);
    println!("{}", s2);
    let s3 = "meow this!";
    println!("{}", first_word_slice(&s3[..]));
    println!("{}", first_word_slice(&s3));

    // Arrays are basically just slices too
    let a = [1,2,3,4,5];
    println!("{:?}", &a[0..3]);

}

// Give function reference to string, return string slice (&str)
// THis function searches for a space in the string and returns the first word found up to the space
// The word returned is a string slice.
// Slice is technically a immutable reference to the original string, this means you cannot make muttable reference
// until the slice is destroyed. Allows for safely detecting problems at compile time, no race conditions allowed.
fn first_word_str (s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s;
}
// Use slices instead of string references
fn first_word_slice (s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s;
}
