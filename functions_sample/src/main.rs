fn main() {
    println!("Hello, world!");
    another_function();
    function_with_parameters(10, 20);

    println!("{}", get_five());
    println!("{}", get_five_with_return());
}

//There is no difference if the function is before or after main
fn another_function() {
    println!("Another function!");
}

fn function_with_parameters(x: i32, y: i32) {
    println!("First parameter = {}", x);
    println!("Second parameter = {}", y);
}

//functions which returns value
//if no 'return' -> no semicolon at the end
fn get_five() -> i32 {
    5
}

//if 'return' -> semicolon at the end
fn get_five_with_return() -> i32 {
    return 5;
}