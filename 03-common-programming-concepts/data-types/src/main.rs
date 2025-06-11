use std::io;

fn main() {
    /*
        Scalar
            Rust has four primary scalar types:
                1. integers
                2. floating-point number
                3. booleans
                4. characters

            Integer Types
                Length	Signed	Unsigned
                8-bit	i8	    u8
                16-bit	i16	    u16
                32-bit	i32	    u32
                64-bit	i64	    u64
                128-bit	i128	u128
                arch	isize	usize

                Integer Literals in Rust
                Number literals	    Example
                Decimal	            98_222
                Hex	                0xff
                Octal	            0o77
                Binary	            0b1111_0000
                Byte (u8 only)	    b'A'
    */

    let x_u: u8 = 255;
    println!("x_u has u8 is: {x_u}");

    let x_i: i8 = -127;
    println!("x_i has i8 is: {x_i}");

    let x_floating = 2.0; // f64 bits
    let y_floating: f32 = 3.0; // f32 bits
    println!("x_floating has f64 is: {x_floating}");
    println!("y_floating has f32 is: {y_floating}");

    /* Numberic Operation*/

    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotien = 56.7 / 32.2;
    let truncated = -5 / 3; // result in -1
    println!("quotien: {quotien}");
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    /* 
        The Boolean Type 

        Boolean are one byte in size

        Comparison Table
        Reason	                Explanation
        Hardware                Addressability	CPUs address memory in bytes, not bits
        Performance	            Byte-wise operations are faster than bitwise manipulations
        Language Guarantees	    Rust and C99 guarantee bool is 1 byte for safety and FFI compatibility
        Pointer Semantics	    Cannot take a pointer to a bit; must be at least a byte
        Compiler Optimizations	Unused bits in the byte can be leveraged for enum or Option optimizations

    */
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t: {t}");
    println!("f: {f}");

    /* 
        The Character Type

        char type is four bytes in size and represents a Unicode Scalar Value
    */
    
    let c = 'z'; // must use single quotes
    let z: char = 'Z'; // with explicit type annotation
    let thai_char: char = 'ก'; // unicode
    println!("c: {c}");
    println!("z: {z}");
    println!("thai_char: {thai_char}");


    /* 
        Compound Types 

        compound types can group multiple values into one type

        2 types:
            - tuples
            - arrays
    */

    /* 
        The Tuple Type 
        
        a tuple is a general way of grouping together a number of values with a variety of types into one compound type
        tuples have a fixed length: once declared, they cannot grow or shrink in size

        tuple element directly by suing a period (.) followed by the index of the value

        the first in a tuple is 0
    */

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup_2 = (500, 6.4, 1, 'c', "Thai");
    let (_x, y, _z, c, s) = _tup_2;

    println!("The value of y is: {y}");
    println!("The value of c is: {c}");
    println!("The value of s is: {s}");
    
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    /* 
        Array Type 

        another way to collection of multiple values is with an array.
        every element of an array must have the same type.
        array in Rust have a fixed length.

        However, arrays are more useful when you know the number of elements will not need to change.

        sample:
            let a: [i32; 5] = [1, 2, 3, 4, 5];

            i32 is the type of each element. after the semicolon
            the 5 indicates the array contains five elements


            let a = [3; 5] 

            the array named a will contain 5 elements that will all be set to the value 3 initially.
            this is the same as writing let a = [3, 3, 3, 3, 3]
    */

    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                    "August", "September", "October", "November", "December"];

    let first = a[0];
    let second = a[1];

    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    println!("The value of a is: {}", a[3]);
    println!("The value of b is: {}", b[4]);
    println!("The value of c is: {}", c[3]);
    println!("The value of months is: {}", months[8]);

    let length_of_a: usize = a.len();
    println!("The value of length a is: {length_of_a}");

    println!("Please enter an array index. between 0 to 4 ?");
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
    
}
