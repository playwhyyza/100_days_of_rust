fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    
    s.clear(); // this empties the String, marking it equal to ""

    /*
        `word` still has the value `5` here, but `s` no longer has any content
        that we could meaningfully use with the value `5`, so `word` is now
        totally invalid!
    */

    println!("space index of s: {word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert our String to an array of bytes

    /*
        create an iterator over the array of bytes.
        &item for the single byte in the tuple.
        Because we get a reference to the element from .iter().enumerate(),
        we use `&` in the pattern
     */
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' { // search for the byte that represents the space by using the byte literal syntax
            return index; // return the position
        }
    }
    
    s.len() // Otherwise, return the length of the string by using `s.len`
}