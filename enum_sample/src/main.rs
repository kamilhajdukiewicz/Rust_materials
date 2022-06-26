#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message info: {:#?}", self);
    }
}

fn main() {
    let ip1 = IpAddrKind::V4(String::from("localhost"));
    println!("IP1 = {:#?}", ip1);

    let message1 = Message::Quit;
    let message2 = Message::Move {
        x: 10,
        y: 20,
    };

    message1.call();
    message2.call();
}
