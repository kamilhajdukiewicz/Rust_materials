use std::io;

fn main() {
    let mut number = String::new();
    println!("Input the number = ");

    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read the line");

    let mut number : i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let is_equal_10 = if number == 10 { true } else { false };
    println!("Is the number equal to 10? - {}", is_equal_10);

    'outside_loop_sample : loop {
        loop {
            if number > 10 {
                println!("Value is greater than 10.");
                number = number - 1;
            } else if number == 10 {
                println!("Value is equal to 10.");
                break 'outside_loop_sample;
            } else {
                println!("Value is lower than 10.");
                number += 1;
            }
        }
        //println!("We are still inside the first loop!");
    }

    let mut counter = 0;
    let value_from_loop : i32 = loop {
        if counter == 10 {
            break counter * 2
        }
        counter += 1;
    };
    println!("Value from loop = {}", value_from_loop);

    while counter > 0 {
        println!("Counter = {}", counter);
        counter -= 1;
    }

    // iterating through collection
    let array_sample = [1,2,3,4,5];
    let mut index = 0;

    while index < 5 {
        println!("Element at index {} = {}", index, array_sample[index]);
        index += 1;
    }
    
    for element in array_sample {
        println!("Elements in array = {}", element);
    }
}
