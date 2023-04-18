use rand::Rng; //random number generator trait (must be in scope for methods to work)
use std::io; //standard library //third party crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //1..=100 - end of Range is inclusive
    println!("The secret number is {}.", secret_number);

    println!("Please input your guess.");

    //creates a new instance of a String - growable UTF-8 encoded text
    // :: - Associated Function (function implemented on a type)
    let mut guess = String::new();

    //input/output::standard_input()
    io::stdin()
        //(...) - where to append the input
        //&mut guess - mutable reference (like a copy without memory use)
        .read_line(&mut guess)
        //called on the read_line return value Result
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
