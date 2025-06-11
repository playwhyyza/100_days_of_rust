fn main() {
    /* if expressions */

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*
        Rust expected a bool but got an integer.
        Unlike language such as Ruby, Python and JavaScript,
        Rust will not automatically try to convert non-Boolean types to a Boolean
        You must be explicit and always provide if with a Boolean as its condition.
     */

    if number != 0 {
        println!("number was something other than zero");
    }

    // Handling Multiple Conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // Using if in a let statement

    let condition = true;
    
    /*
        if must be the same type;
        
        The expression in the if block evaluates to an integer, 
        and the expression in the else block evaluates to a string. 
        This won’t work because variables must have a single type, 
        and Rust needs to know at compile time what type the number variable is, 
        definitively. Knowing the type of number lets the compiler verify the type is valid everywhere we use number. 
        Rust wouldn’t be able to do that if the type of number was only determined at runtime;
        the compiler would be more complex and would make fewer guarantees 
        about the code if it had to keep track of multiple hypothetical types for any variable.
     */

    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {number}");

    // Repetition with Loops

    /* 
        Repeating Code with loop

        The Loop keyword tells Rust to execute 
        a block of code over and over again foever 
        or until you explicitly tell it to stop.


        Rust provides a way to break out of a loop using code.
        You can place the `break` keyword within the loop to tell
        the program when to stop executing the loop.

        We also used `continue` in a loop tell the program to skip over any
        remaining code in this iteration of the loop and go the next iteration.
     */

    // Returning Values from Loops

    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops

    /*
        The outer loop has the label 'counting_up, 
        and it will count up from 0 to 2. 
        The inner loop without a label counts down from 10 to 9. 
        The first break that doesn’t specify a label will exit the inner loop only. 
        The break 'counting_up; statement will exit the outer loop. 
     */

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1
    }
    println!("End count = {count}");
}
