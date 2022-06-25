//Added attribute which allows you to print struct in println!
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Method implementation
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    //method without self
    //Can be executed like that Rectangle::create_square(size: u32)
    fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 20,
        height: 50,
    };

    let rec3 = Rectangle {
        width: 50,
        height: 10,
    };

    //Different ways to print rec value
    println!("Rect1 is {:#?}", rec);
    println!("Rect1 is {:?}", rec);
    dbg!(&rec);
    
    //function call
    println!("The area of the rectangle is {} square pixels.", area(&rec));

    //method call
    println!("The area of the rectangle is {} square pixels.", rec.area());

    println!("Can rec1 hold rec2? -? {}", rec.can_hold(&rec2));
    println!("Can rec1 hold rec3? -? {}", rec.can_hold(&rec3));

    let square = Rectangle::create_square(10);
    println!("Square: {:#?}", square);
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}