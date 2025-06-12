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
        Loop labels must begin with a single quote. 

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

    // Conditional Loops with while

    /*
        A program will often need to evaluate a condition within a loop. 
        While the condition is true, the loop runs. 
        When the condition ceases to be true, 
        the program calls break, stopping the loop. 
        It’s possible to implement behavior like this using a combination of loop, 
        if, else, and break; 
     */

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!");

    // Looping Through a Collection with for

    /*
        the code counts up through the elements in the array. 
        It starts at index 0, and then loops until it reaches the final index in the array 
        (that is, when index < 5 is no longer true). 
        Running this code will print every element in the array:

        but forgot to update the condition, the code would panic.
        It’s also slow, because the compiler adds runtime code to perform the conditional 
        check of whether the index is within the bounds of the array on every
        iteration through the loop.
     */

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    /* 
        more concise alternative, you can use a for loop and execute some code 
        for each time in a collection

        we’ve now increased the safety of the code and eliminated the chance of 
        bugs that might result from going beyond the end of the array or not going 
        far enough and missing some items.

        Using the for loop, you wouldn’t need to remember to change any other 
        code if you changed the number of values in the array
    */

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    /*
        most Rustaceans would use a for loop. 
        The way to do that would be to use a Range, provided by the standard library, 
        which generates all numbers in sequence starting from one number and ending 
        before another number.

        Here’s what the countdown would look like using a for loop and another method 
        we’ve not yet talked about, `rev``, to reverse the range:

        loop between 1 to 3
     */

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
