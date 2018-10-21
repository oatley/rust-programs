struct Rectangle (u32, u32);
struct Rect {
    width: u32,
    height: u32,
}
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area in the rectangle is {} square pixels", area(width1, height1));

    let dimensions = (30, 50);
    println!("The area in the rectangle is {} square pixels", area2(&dimensions));

    let rectangle = Rectangle(30,50);
    println!("The area in the rectangle is {} square pixels", area3(&rectangle));

    let rect = Rect {width: 30, height: 50};
    println!("The area in the rectangle is {} square pixels", area4(&rect));


}

fn area (width: u32, height: u32) -> u32 {
    width * height
}

fn area2 (dimensions: &(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3 (rectangle: &Rectangle) -> u32 {
    rectangle.0 * rectangle.1
}

fn area4 (rect: &Rect) -> u32 {
    rect.width * rect.height
}
