/*
    Rust won’t let us annotate a type with Copy if the type, 
    or any of its parts, has implemented the Drop trait. 
    If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, 
    we’ll get a compile-time error. 
    To learn about how to add the Copy annotation to your type to implement the trait, 
    see “Derivable Traits” in Appendix C.

    So, what types implement the Copy trait? You can check the documentation for the given type to be sure, 
    but as a general rule, any group of simple scalar values can implement Copy, 
    and nothing that requires allocation or is some form of resource can implement Copy. 
    Here are some of the types that implement Copy:

    All the integer types, such as u32.
    The Boolean type, bool, with values true and false.
    All the floating-point types, such as f64.
    The character type, char.
    Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.


*/

fn main() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}
