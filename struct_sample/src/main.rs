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

    //struct update syntax, onyl email changed, rest of the memebers are the same as in user1
    //now the user1 is not valid. The ownership was moved to user1_updated
    let user1_updated = User {
        email: String::from("ragnartrump@gmail.com"),
        ..user1
    };
    //println!("User1 = {}, user1.username")
    println!("User1 = {}", user1_updated.username);

    let color = Color(255,255,0);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//struct in tuple style
struct Color(u32, u32, u32);

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