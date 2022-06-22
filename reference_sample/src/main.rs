fn main() {

    //basic reference
    let str = String::from("Hello, world!");
    println!("Length of the text is {}", calculate_length(&str));


    //mutable reference sample
    let mut mut_str = String::from("Base text");
    println!("Text before the change = {}", mut_str);
    change_text(&mut mut_str);
    println!("Text after change = {}", mut_str);
}

fn calculate_length(s: &String) -> usize {  //the function takes the reference. str is still owner.
    s.len()
}

//mutable reference
fn change_text(s: &mut String) {
    s.push_str(" after change");
}