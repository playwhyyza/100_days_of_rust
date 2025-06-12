/*
    If we do want to deeply copy the heap data of the String, 
    not just the stack data, we can use a common method called clone. 
*/

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
