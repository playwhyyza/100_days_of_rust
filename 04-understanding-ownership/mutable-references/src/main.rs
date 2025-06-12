/*
    allow us to modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:

    First we change s to be mut. Then we create a mutable reference with &mut s where we call the change function, 
    and update the function signature to accept a mutable reference with some_string: &mut String. 
    This makes it very clear that the change function will mutate the value it borrows.

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);

    The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. 
    It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. 
    The benefit of having this restriction is that Rust can prevent data races at compile time. 
    A data race is similar to a race condition and happens when these three behaviors occur:

    Noted:
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data.
    - There’s no mechanism being used to synchronize access to the data.

    Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!
*/

fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("the value of s is: {s}");

    /*
        As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
     */
    {
        let r1 = &mut s;
        println!("r1: {r1} << inside curly brackets");
    } // r1 goes out of scope here, so we can make a new reference with no problem.

    let r2 = &mut s;

    println!("r2: {r2}");

    // combinding mutable and immutable references
    let r3 = &s; // no problem
    let r4 = &s; // no problem 
    
    /* 
        Noted:
            Whew! We also cannot have a mutable reference while we have an immutable one to the same value.
    */

    // let r5 = &mut s; // BIG PROBLEM

    println!("r3: {r3}, r4: {r4}");
    // variable r3 and r4 will not be used after this point.

    /*
        Noted:
            The scopes of the immutable references r3 and r4 end after the println! where they are last used, 
            which is before the mutable reference r5 is created. 
            These scopes don’t overlap, 
            so this code is allowed: the compiler can tell that the reference is no longer being used at a point before the end of the scope.
     */
    
    println!("after immutable references println");
    let r5 = &mut s; // BIG PROBLEM
    
    println!("r5: {r5}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
