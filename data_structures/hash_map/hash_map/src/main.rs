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



}
