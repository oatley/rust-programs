enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let x = Coin::Penny;
    let y = Coin::Quarter;
    check(x);
    check(y);
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn check(coin: Coin) {
    match coin {
        Coin::Penny => println!("1"),
        Coin::Nickel => println!("5"),
        Coin::Dime => println!("10"),
        Coin::Quarter => println!("25"),
    }
}
