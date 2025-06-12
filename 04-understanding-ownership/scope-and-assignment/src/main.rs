/*
    The inverse of this is true for the relationship between scoping, 
    ownership, and memory being freed via the drop function as well.
    When you assign a completely new value to an existing variable, 
    Rust will call drop and free the original value’s memory immediately.
*/

fn main() {
    let mut s = String::from("hello");
    println!("{s}, World!");
    
    s = String::from("ahoy"); // drop and free the origin value memory immediately
    println!("{s}, World!");
}
