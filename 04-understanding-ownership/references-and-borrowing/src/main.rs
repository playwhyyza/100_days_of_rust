/*

    First, notice that all the tuple code in the variable declaration and the function return value is gone. Second, 
    note that we pass &s1 into calculate_length and, in its definition, 
    we take &String rather than String. These ampersands represent references, 
    and they allow you to refer to some value without taking ownership of it.

*/

fn main() {
    let s1 = String::from("hello");

    // look like pass by reference in C++
    let len = calculate_length(&s1); // &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
                                    // because the reference does not own it, the value it points to will not be dropped when the references stoip being used.
    println!("The length of '{s1}' is {len},");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what it refers to, the value is not dropped.
