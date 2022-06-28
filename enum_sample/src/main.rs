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

    //Option sample
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five = {:#?}, Six = {:#?}, None = {:#?}", five, six, none); 

    let state = State::Texas;
    println!("Are you in Alabama? - {:?}", are_you_in_alabama(state));
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

fn plus_one(opt: Option<i32>) -> Option<i32> {
    match opt {
        None => None,
        Some(opt) => Some(opt + 1),
    }
}

fn are_you_in_alabama(state: State) -> bool {
    match state {
        State::Alabama => true,
        //this handle any other case, _ -> means that the value is not needed.
        _ => false,
    }
}