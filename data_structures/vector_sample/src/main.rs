fn main() {
    //creating a empty vector which stores int values
    let mut vec1 : Vec<i32> = Vec::new();

    //creating a vector which stores 1,2,3
    let vec2 = vec![1,2,3];

    //adding element to vector
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);

    //removing element from vector
    vec1.pop();

    //reading an item from vector using brackets
    let third_element = &vec1[2];
    println!("Third element = {}", third_element);

    //reading an item from vector using get method
    match vec2.get(1) {
        Some(value) => println!("The second element is {}", value),
        None => println!("There is no second element"),
    }

    //it possible to try acces an element outisde the range using get
    match vec2.get(5) {
        Some(value) => println!("The sixth element is {}", value),
        None => println!("There is no sixth element"),
    }

    //iterating throught the collection
    for item in &vec1 {
        println!{"Value = {}", item};
    }

    for item in &mut vec1 {
        *item = *item + 50;
        println!("Value after change = {}", item);
    }

    //Vector which stores different types
    let vec3 = vec![EnumTypes::Int(10), 
                    EnumTypes::Float(10.5),
                    EnumTypes::Text(String::from("Text"))];

    for item in &vec3 {
        println!("Element = {:#?}", item);
    }
}

#[derive(Debug)]
enum EnumTypes {
    Int(i32),
    Float(f32),
    Text(String),
}
