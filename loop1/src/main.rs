fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter;
        }
    };
    assert_eq!(result, 11);
    println!("Result = {}", result);
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
