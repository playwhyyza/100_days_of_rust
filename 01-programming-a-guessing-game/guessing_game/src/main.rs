use std::io; // bring the io input/output library into scrope

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // line has created a mutable variable that is currently bound to a new, empty instance of String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // std::io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let apple = 5; // immutable
    let mut banana = 5; // mutable

    banana += 1; // now, banana equal to 6

    println!("apple: {apple}, banana: {}", banana + 1);
}
