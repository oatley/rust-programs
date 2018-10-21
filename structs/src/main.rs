// If you want to change anything inside the entire instance must be muttable (struct mut User)
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("kitty@meowmeow.ca"),
        username: String::from("kitty"),
        active: true,
        sign_in_count: 1,
    };
    println!("username={}, email={}, domain={}",user1.username, user1.email, get_domain(&user1.email[..]));
    let user2 = new_user(String::from("dog"), String::from("dog@woofwoof.ca"));
    println!("username={}, email={}, domain={}",user2.username, user2.email, get_domain(&user2.email[..]));

    // Struct update syntax syntax shorthand
    let user2 = User {
        username: String::from("bird"),
        email: String::from("bird@chirpchirp.ca"),
        ..user2
    };
    println!("username={}, email={}, domain={}",user2.username, user2.email, get_domain(&user2.email[..]));


}

// Create a new user and return the struct
fn new_user(username: String, email: String) -> User {
    // Field init shorthand allows you to just use a variable if it has same field name
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

// Loop through a slice of a string and search for @, extract domain from email
fn get_domain(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'@' {
            return &s[i+1..];
        }
    }
    return &s[..];
}
