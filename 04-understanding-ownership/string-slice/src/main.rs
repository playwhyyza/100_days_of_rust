/*
    Note: String slice range indices must occur at valid UTF-8 character boundaries. 
    If you attempt to create a string slice in the middle of a multibyte character, 
    your program will exit with an error. For the purposes of introducing string slices,
    we are assuming ASCII only in this section
*/

fn main() {
    let s = String::from("hello, world");

    /*
        We create slices using a range within brackets by specifying
        [starting_index..ending_index], where `starting_index` is the first position by
        the slice and `ending_index` is one more than the last position in the slice
     */
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {hello}");
    println!("world: {world}");

    let s = String::from("hello");

    // if you want to start at index 0, you can drop the value before the two periods
    let slice_option_1 = &s[0..2];
    let slice_option_2 = &s[..2];
    
    println!("slice_option_1: {slice_option_1}");
    println!("slice_option_2: {slice_option_2}");

    // if you slice includes the last byte of the `String`, you can drop the trailling number
    let len = s.len(); // length of s

    let slice_option_3 = &s[3..len];
    let slice_option_4 = &s[3..];

    println!("slice_option_3: {slice_option_3}");
    println!("slice_option_4: {slice_option_4}");

    // drop both values to take a slice of the entire string
    let slice_option_5 = &s[0..len];
    let slice_option_6 = &s[..];

    println!("slice_option_5: {slice_option_5}");
    println!("slice_option_6: {slice_option_6}");

    let s = String::from("hello world");
    
    let word = first_word(&s);

    // s.clear(); // error!
    
    println!("the first word is: {word}");
}

fn first_word(s: &String) -> &str { //  “string slice” is written as &str
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index]; // return position 0 as long as no space character is found.
        }
    }

    &s[..] // return entire string
}
