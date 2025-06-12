fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back,
                                        // which also moves its return value into s3
    println!("s1: {s1}");
    println!("s3: {s3}");

    let (s2, len) = calculate_length(s1);   // s1 is move into calculate_length, which also moves its return value into s2 and len

    println!("The length of '{s2}' is {len}.")
}   // here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens.
    // s1 goes out of scope and is dropped.

fn gives_ownership() -> String {                // gives_ownershio will move its return value into the function that call it
    let some_string = String::from("your");     // some_string comes into scope

    some_string                                 // some_string is returned and moves out to the calling function
}

// This functions takes a String and returns a String
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}