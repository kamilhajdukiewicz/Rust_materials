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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    Alaska,
    Alabama,
    Texas,
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

    let coin = Coin::Quarter(State::Texas);
    println!("Value of the coin is {}", value_in_cents(coin));
}

//match function sample
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The quarter is from state {:#?}", state);
            25
        }
    }
}