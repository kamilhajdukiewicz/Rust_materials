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

    let user2 = build_user(String::from("User2"), String::from("user2@gmail.com"));
    println!("User2 username: {}, email: {}", user2.username, user2.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//If the parameter have the same name as the member of the struct
//You can write only the name -> not username: username
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}