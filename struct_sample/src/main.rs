fn main() {

    //instance of the user struct
    let mut user1 = User {
        active: false,
        username: String::from("RagnarTrump"),
        email: String::from("myemail@gmail.com"),
        sign_in_count: 0,
    };

    println!("Username before change: {}", user1.username);

    user1.username = String::from("NewUsername");

    println!("Username after change: {}", user1.username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}