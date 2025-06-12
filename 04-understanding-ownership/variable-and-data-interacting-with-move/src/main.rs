/*
    s1
    ----------------                -------------------
    |name    |value|                | index  |  value |
    ----------------                -------------------
    |prt     | ----|------------>   |   0    |    h   |
    ----------------                -------------------
    |len     |  5  |                |   1    |    e   |
    ----------------                -------------------
    |capacity|  5  |                |   2    |    l   |
    ----------------                -------------------
                                    |   3    |    l   |
                                    -------------------
                                    |   4    |    o   |
                                    -------------------

    
    The length is how much memory, in bytes, 
    the contents of the String are currently using. 
    The capacity is the total amount of memory, in bytes, 
    that the String has received from the allocator. 
    The difference between length and capacity matters, but not in this context, 
    so for now, it’s fine to ignore the capacity.

    When we assign s1 to s2, the String data is copied, meaning we copy the pointer, 
    the length, and the capacity that are on the stack. 
    We do not copy the data on the heap that the pointer refers to
    
    If you’ve heard the terms shallow copy and deep copy while working with other languages, 
    the concept of copying the pointer, 
    length, and capacity without copying the data probably sounds like making a shallow copy. 
    But because Rust also invalidates the first variable, 
    instead of being called a shallow copy, 
    it’s known as a move. In this example, 
    we would say that s1 was moved into s2

    That solves our problem! With only s2 valid, 
    when it goes out of scope it alone will free the memory, 
    and we’re done.

    In addition, there’s a design choice that’s implied by this: 
    Rust will never automatically create “deep” copies of your data. 
    Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
*/

fn main() {
    let x = 5;
    let y = x;
    println!("the value of x is: {x}");
    println!("the value of y is: {y}");

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("the value of s1 is: {s1}");
    println!("the value of s2 is: {s2}");
}
