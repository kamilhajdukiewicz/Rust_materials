//Added attribute which allows you to print struct in println!
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    //Different ways to print rec value
    println!("Rect1 is {:#?}", rec);
    println!("Rect1 is {:?}", rec);
    dbg!(&rec);
    println!("The area of the rectangle is {} square pixels.", area(&rec));
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}