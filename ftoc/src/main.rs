// Convert Fah to Cel
use std::io;

fn main() {
    loop {
        println!("Please enter a number: ");
        let mut fah = String::new();
        io::stdin().read_line(&mut fah).expect("Not good enough");

        let fah: i32 = match fah.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number... Try again... Idiot:");
                continue;
            }
        };
        let cel: i32 = (fah - 32) * 5 / 9;
        println!("Fahrenheit: {} Celcius: {}", fah, cel);
        break;
    }
}
