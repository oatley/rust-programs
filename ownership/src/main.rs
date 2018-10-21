fn main() {
    let s1 = String::from("meow");
    take_ownership1(s1);
    
    // This will fail because function takes ownership
    //println!("{}", s1);

    // The function takes ownership so s2 stops existing
    // The function then returns the string back into this scope using shadowing
    //let s2 = String::from("meow");
    let s2 = give_ownership();
    let s2 = take_ownership2(s2);
    println!("{}", s2);
    take_ownership3(&s2)
}

// This function will take ownership of string and never give it back
// s1 is moved from main scope to function and destroyed in main scope
fn take_ownership1(s: String) {
    println!("{}", s);

} // s will be destroyed/dropped at end of function

// This function will take ownership and then give it back
fn take_ownership2(s: String) -> String {
    println!("{}", s.len());
    s
}

// This function will not take ownership, instead it accepts a reference as argument
// The data in the reference can be read, but not changed, because the function doesn't own it
// The reference is dropped at the end without modifying or changing the data on the heap
fn take_ownership3(s: &String) {
    println!("{}", s.len());
}

// Gives meows to everyone
// Creates completely new string and returns the ownership to where function is called
fn give_ownership() -> String {
    String::from("meow meow meow")
}








