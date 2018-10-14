fn main() {

    // Arrays are fixed length, same type items
    let arrr = [1,2,3,4,5];
    println!("First: {}, Last: {}", arrr[0], arrr[4]);

    // Set a specific type with syntax [type; length]
    let arrr2: [i64; 3] = [10,11,12];
    println!("First: {}, Last: {}", arrr2[0], arrr2[2]);
    

}
