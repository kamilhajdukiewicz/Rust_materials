fn main() {
    println!("Hello, world!");
    another_function();
    function_with_parameters(10, 20);
}

//There is no difference if the function is before or after main
fn another_function() {
    println!("Another function!");
}

fn function_with_parameters(x: i32, y: i32) {
    println!("First parameter = {}", x);
    println!("Second parameter = {}", y);
}