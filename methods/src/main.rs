#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
    // accociated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    println!("The area of rect is {:?} square pixels", rect1.area());
    println!("The area of rect is {:?} square pixels", rect2.area());
    println!("The area of rect is {:?} square pixels", rect3.area());
    println!("Can rect1 hold rect2? {} ", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {} ", rect1.can_hold(&rect3));
    // This is a accociated functions(not a method), usually used for constructors, does not pass or use self
    let square = Rectangle::square(10);
    println!("The area of rect is {:?} square pixels", square.area());

}
