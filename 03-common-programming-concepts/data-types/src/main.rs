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

    // The Tuple Type
    

    // The Array Type

}
