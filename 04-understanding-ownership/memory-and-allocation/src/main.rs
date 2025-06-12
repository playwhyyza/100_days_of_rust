/*

    There is a natural point at which we can return the memory our String 
    needs to the allocator: when s goes out of scope. 
    When a variable goes out of scope, Rust calls a special function for us. 
    This function is called drop, and it’s where the author of String 
    can put the code to return the memory. 
    Rust calls drop automatically at the closing curly bracket.

*/

fn main() {
    let s = String::from("Hello");  // s is valid from this point forward
    
    // do stuff with s
    println!("the value of s is: {s}");
}                                   // this scope is now over, and s is no longer valid
