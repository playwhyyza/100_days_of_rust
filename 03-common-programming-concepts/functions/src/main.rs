fn main() {
    println!("Hello, world!");
    
    another_function(32); // pass 32 into function
    
    print_labeled_measurement(5, 'h');
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