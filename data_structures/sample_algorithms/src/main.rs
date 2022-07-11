use std::collections::HashMap;

fn main() {
    //First 
    let mut values = vec![1, 45, 6, 4, 6, 7, 8, 9, 2, 3, 4, 5, 5, 8, 6, 5, 2, 98];
    let median = get_median(&mut values);
    let mode = get_mode(&values);

    //Second
    let mut pig_latin1 = String::from("aye");
    string_to_pig_latin(&mut pig_latin1);
    let mut pig_latin2 = String::from("banana");
    string_to_pig_latin(&mut pig_latin2);
    println!("1. aye -> {}", pig_latin1);
    println!("2. banana -> {}", pig_latin2);
}

//1.
//Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode 
//(the value that occurs most often; a hash map will be helpful here) of the list.
fn get_median(integers: &mut Vec<i32>) -> i32 {
    integers.sort();
    let size = integers.len();
    let median = integers[size/2];
    println!("After sorting: {:?}", &integers);
    println!("Median is {}", &median);
    median
}

fn get_mode(integers: &Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    let mut max_key : i32 = 0;
    for int in integers {
        let count = hm.entry(int).or_insert(0);
        *count += 1;
        if *count > max_key {
            max_key = *int;
        }
    }
    println!("Mode is {}", max_key);
    max_key
}

//2.
//Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, 
//so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
//Keep in mind the details about UTF-8 encoding!
fn string_to_pig_latin(str : &mut String) {
    if str.starts_with(&['a', 'e', 'i', 'o', 'u']) {
        str.push_str("-hay");
    } else {
        str.push('-');
        str.push(str.chars().next().unwrap());
        str.push_str("ay");
        str.remove(0);
    }
}
