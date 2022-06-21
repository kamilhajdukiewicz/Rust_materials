fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let s2 = String::from("Hello");             
    takes_ownership(s2.clone());                //s2 is cloned into the function (deep copy)
    println!("String is still valid - {}", s2); //s2 clone is valid only inside hte function, but the original s2 is still
                                                //valid inside the main function scope

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    println!("{}", x);

    let str1 = gives_ownership();
    println!("String from function - {}", str1);

    let str2 = takes_and_gives_back(str1);
    //println!("Str1 = {}", str1);
    println!("Str2 = {}", str2); 

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {            // gives_ownership will move its
                                            // return value into the function
                                            // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                             // some_string is returned and
                                            // moves out to the calling
                                            // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

a_string  // a_string is returned and moves out to the calling function
}
