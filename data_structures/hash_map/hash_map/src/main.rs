use std::collections::HashMap;

fn main() {

    //creating a hash map
    let mut hm = HashMap::new();

    //adding elements to hash map
    hm.insert(String::from("Yellow"), 10);
    hm.insert(String::from("Red"), 50);

    //creating a hash map using vectors
    let teams = vec![String::from("Blue"), String::from("Green")];
    let points = vec![10, 50];

    let mut hm2 : HashMap<_ , _> = teams.into_iter().zip(points.iter()).collect();

    //reading data from hash map
    match hm.get(&String::from("Yellow")) {
        Some(point) => println!("Yellow team points = {}", point),
        None => println!("Yellow team doesn't have points"),
    }
    
    //iterating through all elements
    for (key, value) in hm2 {
        println!("{} team points = {}", key, value);
    }

    //overwriting the value
    hm.insert(String::from("Black"), 20);
    println!("{:?}", hm);
    hm.insert(String::from("Black"), 25);
    println!("{:?}", hm);

    //inserting value only wen the key has no value yet
    hm.entry(String::from("White")).or_insert(99);
    hm.entry(String::from("Black")).or_insert(90);
    println!("{:?}", hm);

    //updating the value based on the old value
    let text = "Hello world, Hello earth!";
    let mut word_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_map);
}
