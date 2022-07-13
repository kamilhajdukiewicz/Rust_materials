use std::fs::File;

fn main() {
    //RUST_BACKTRACE=1 cargo run to see callstack 
    //panic!("Crash!");

    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => panic!("File cannot be opened: {:#?}", error),
    };
}
