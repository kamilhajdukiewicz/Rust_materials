fn main() {

    //basic reference
    let str = String::from("Hello, world!");
    println!("Length of the text is {}", calculate_length(&str));


    //mutable reference sample
    let mut mut_str = String::from("Base text");
    println!("Text before the change = {}", mut_str);
    change_text(&mut mut_str);
    println!("Text after change = {}", mut_str);

    //----This code below is not allowed---
    //you can have only one mutable reference to a particular piece of data at a time.
    //The benefit of having this restriction is that Rust can prevent data races at compile time
    //let s1 = &mut mut_str;
    //let s2 = &mut mut_str;
    //println!("s1 {}, s2 {}", s1, s2);

    //But this is allowed, cause the value can't be modified by s1 or s2
    let s1 = &mut_str;
    let s2 = &mut_str;
    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = &mut mut_str;
    println!("s3 = {}", s3);

}

fn calculate_length(s: &String) -> usize {  //the function takes the reference. str is still owner.
    s.len()
}

//mutable reference
fn change_text(s: &mut String) {
    s.push_str(" after change");
}