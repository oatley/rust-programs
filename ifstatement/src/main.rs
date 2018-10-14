fn main() {
    let x = 5;

    if x > 5 {
        println!("big");
    } else if x == 5 {
        println!("just right");
    } else {
        println!("small");
    }

    let y = if x == 5 { 10 } else { 11 };
    println!("Meow: {}", y);

}
