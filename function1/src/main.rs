fn main() {
    println!("Hello, world!");
    a1();
    a2(5);
    
    // Super explicit conversion expression
    let a: u8 = 257i32 as u8 ;
    println!("Meow: {}", a);

    let b: u8 = fifty() as u8;
    println!("Meow: {}", b);
}

fn a1() {
    println!("Hello, world!");
}

fn a2(x: i32) {
    println!("AHHHHHH: {}", x + 100);
}

fn fifty() -> i32 { 
    50
}
