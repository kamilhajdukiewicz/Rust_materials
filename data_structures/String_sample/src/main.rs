fn main() {
   
    //creating a String variable
    let mut str1 : String = String::new();

    let data = "Data for String";

    //fill the String with some text data
    let str2 = data.to_string();
    let str3 = String::from(data);

    println!("1. {}", str2);
    println!("2. {}", str3);

    //updating a String with string
    str1.push_str("Hello");
    println!("Updated string: {}", str1);

    //updating a string with character
    str1.push('!');
    println!("After push: {}", str1);

    //concatenate strings with + operator
    let world = String::from(" World!");
    let str4 = str1 + &world;
    //str1 is not valid anymore
    println!("After + : {}", str4);

    //different way to combining strings
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let ttt = format!("{}-{}-{}", tic, tac, toe);
    println!("ttt = {}", ttt);


    //iterating through strings
    let s = String::from("नमस्ते");

    //chars
    for c in s.chars() {
        println!("{}", c);
    }

    //bytes
    for c in s.bytes() {
        println!("{}", c);
    }
}
