fn main() {
    // An enum can be passed into a function
    // But it can have a single value set inside it, which make it able to be multiple type
    enum Message {
        Quit,
        Move {x: i32, y:i32},
        Write(String),
        ChangeColour(i32,i32,i32),
    }

    // Options are similar to enums but have a value or none, used to replace null values
    // Since no other value in rust can be null
    impl Message {
        fn call(&self) {
            println!("meow");
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

}
