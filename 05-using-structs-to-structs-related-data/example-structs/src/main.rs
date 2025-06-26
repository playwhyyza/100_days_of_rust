/*
    Rust does include functionality to print out debugging information, 
    but we have to explicitly opt in to make that functionality available for our struct. 
    To do that, we add the outer attribute #[derive(Debug)] just before the struct definition
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    /*
        Our area function is now defined with one parameter, 
        which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance. 
        As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. 
        This way, main retains its ownership and can continue using rect1, 
        which is the reason we use the & in the function signature and where we call the function.
     */

    println!("rectangle -> width: {0} | height: {1}", rect1.width, rect1.height);

    println!("rectangle is {rect1:?}"); // plaintext style
    println!("rectangle is {rect1:#?}"); // json style

    /*
        Another way to print out a value using the Debug format is to use the `dbg!` macro, 
        which takes ownership of an expression (as opposed to println!, which takes a reference), 
        prints the file and line number of where that `dbg!`
        macro call occurs in your code along with the resultant value of that expression, 
        and returns ownership of the value.
        
        We can put `dbg!` around the expression 30 * scale, because `dbg!` returns ownership of expression's value

        The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!
     */

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}