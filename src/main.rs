use rand::Rng; //random number generator trait (must be in scope for methods to work)
use std::cmp::Ordering;
use std::io; //standard library //third party crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //1..=100 - end of Range is inclusive

    loop {
        println!("Please input your guess.");

        //creates a new instance of a String - growable UTF-8 encoded text
        // :: - Associated Function (function implemented on a type)
        let mut guess = String::new();

        io::stdin()
            //(...) - where to append the input
            //&mut guess - mutable reference (like a copy without memory use)
            .read_line(&mut guess)
            //called on the read_line return value Result
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        //trim and parse guess into an int
        //shadowing guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //_ is a catchall value - we match all Error values with all contents inside of them
            Err(_) => continue,
        };

        //Compare using match and .cmp()
        //match compares all options and breaks after a successful match
        match guess.cmp(&secret_number) {
            //Ordering enum results of .cmp()
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
        }
    }
}
