fn main() {
    println!("Hello, world!");
    
    another_function(32); // pass 32 into function
    
    print_labeled_measurement(5, 'h');

    // Statement are instructions that perform some action and do not return value.
    let _y = 6;

    // Expression evaluate to a resultant value.
    /*
        As will see below

        is a block that, in this case, evaluates to 4. 
        That value gets bound to y as part of the let statement. 
        Note that the x + 1 line doesn’t have a semicolon at the end, 
        which is unlike most of the lines you’ve seen so far. 
        Expressions do not include ending semicolons. 
        If you add a semicolon to the end of an expression, 
        you turn it into a statement, 
        and it will then not return a value. 
        Keep this in mind as you explore function return values and expressions next.
     */
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Function with Return Values
    let five = five();
    println!("The value of five is: {five}");

    let five_plus = plus_one(5);
    println!("The value of five_plus is: {five_plus}");
}

fn another_function(x: i32) {
    /*
        Rust code uses snake case as the conventional style for function and variable names,
        in which all letters are lowercase and underscores separate words.

        x is specified as 'i32'
        the concrete values are called 'arguments', 
        but is casual conversation, people tend to use the word 'parameter'
     */
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_lable: char) {
    println!("The measurement is: {value}{unit_lable}");
}

fn five() -> i32 {
    /*
        it's work, becuase it's an expression whose value we want to return.
        if you add semicolon after 5, you must use return because it's a statement
     */

    5
}

fn plus_one(x: i32) -> i32 {
    /*
        But if we place a semicolon at the end of the line containing x + 1, 
        changing it from an expression to a statement, we’ll get an error:
     */
    x + 1
}