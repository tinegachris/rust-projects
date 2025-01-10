extern crate rand; // Import the external crate 'rand' for random number generation
use std::io; // Import the standard input/output library
use std::cmp::Ordering; // Import the Ordering enum for comparison results
use rand::Rng; // Import the Rng trait for random number generation

fn main() {
    println!("Guess the number!"); // Print the game introduction message

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100

    loop {
        println!("Please input your guess."); // Prompt the user to input their guess

        let mut guess = String::new(); // Create a mutable String to store the user's guess

        io::stdin()
            .read_line(&mut guess) // Read the user's input and store it in the 'guess' variable
            .expect("Failed to read line"); // Handle any errors that occur during input

        let guess: u32 = match guess.trim().parse() { // Parse the input string into a u32 integer
            Ok(num) => num, // If parsing is successful, use the parsed number
            Err(_) => {
                println!("Please enter a valid number!"); // If parsing fails, prompt the user to enter a valid number
                continue; // Continue to the next iteration of the loop
            }
        };

        println!("You guessed: {}", guess); // Print the user's guess

        match guess.cmp(&secret_number) { // Compare the user's guess to the secret number
            Ordering::Less => println!("Too small!"), // If the guess is less than the secret number, print "Too small!"
            Ordering::Greater => println!("Too big!"), // If the guess is greater than the secret number, print "Too big!"
            Ordering::Equal => {
                println!("You win!"); // If the guess is equal to the secret number, print "You win!"
                break; // Exit the loop
            }
        }
    }
}