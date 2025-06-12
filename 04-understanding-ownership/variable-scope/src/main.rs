fn main() {             // s is not valid here, it's not yet declared
    let s = "hello";    // s is valid from this point forward

    // do stuff this s
    println!("the value of s is: {s}");
}                       // this scope is now over, and s is no longer valid
