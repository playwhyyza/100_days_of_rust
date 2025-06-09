use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        /* 
            convert string to unsigned, 32-bit integer.
            trim
                method on a String instance will eliminate any whitespace at the beginning and end
            parse
                which can only contain numerical data. 
                The user must press enter to satisfy read_line and input their guess,
                which adds a newline character to the string.  
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // value will match the first arm's pattern
            Err(_) => continue, // which tells the programe to go the next iteration of the loop
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}