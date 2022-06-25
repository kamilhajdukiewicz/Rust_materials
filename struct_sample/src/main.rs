fn main() {

    //instance of the user struct
    let user1 = User {
        active: false,
        username: String::from("RagnarTrump"),
        email: String::from("myemail@gmail.com"),
        sign_in_count: 0,
    };

    println!("Hello, world!");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}