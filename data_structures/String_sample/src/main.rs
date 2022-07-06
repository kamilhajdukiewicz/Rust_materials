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
}
