use std::io; // Import the standard input/output library to read user input
use rand::prelude::*; // Import the rand crate to generate random numbers
use std::cmp::Ordering; // Import the Ordering enum to compare values

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {}", secret_number);

    // Start a loop to allow the user to guess multiple times
    loop {
        println!("Please input your guess!");

        // Create a mutable String to store the user's guess
        let mut guess = String::new();
    
        // Read the user's input from standard input and store it in the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Handle any errors that may occur while reading the input

        // Convert the user's guess from a String to a u32 (unsigned 32-bit integer)
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num, // If the conversion is successful, store the number in guess_number
            Err(_) => { // If the conversion fails (e.g., the user entered a non-numeric value), print an error message and continue the loop
                println!("Please type a number!");
                continue;
            }
        };
    
        println!("You guessed: {guess_number}");
        
        // Compare the user's guess with the secret number and provide feedback
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), //when the guess is less than the secret number
            Ordering::Greater => println!("Too big!"), //when the guess is greater than the secret number
            Ordering::Equal => { //when the guess is equal to the secret number
                println!("You win!, the secret number is: {}", secret_number);
                break; // Exit the loop if the user guessed the correct number
            }
                
        }
    }
}