use std::io; // library for standard input and output operations
use rand::Rng; // library for to generate random numbers, the Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use std::cmp::Ordering; // library for comparing two values

fn main() {
    println!("Guess the number! "); //println! is a macro
    let secret_number = rand::thread_rng().gen_range(1, 101); // secret_number variable is not mutable because secret number is fixed 
    // println!("The secret number is: {}", secret_number);
    let mut count = 0;
    loop{ // Loops until the user guesses the number
        println!("Please input your guess: ");
        let mut guess = String::new(); // defining a mutable string empty variable guess, as variables are by default immutable in rust, create a String object, "::" - means that new is an associated function
        // of the String type (static method as it is implemented on the type String and not its instance)
        io::stdin().read_line(&mut guess) // gets input from user, passing mutable reference of guess (as references are immutable by default in rust), which means borrowing in rust, and prints the expect string if reading failed for any reason
        .expect("Failed to read number"); // read_line returns io::Result type (enums), the variants for the enum here are "Ok"(indicates success) and "Err" (indicates failure), if "Err" then expect will cause the program to
        // crash and display the message that we have passed as an argument to the expect function, if "Ok" then value is the number
        // of bytes in what the user entered into standard input 
        let guess: u32 = match guess.trim().parse() { // Handles Result type returned by parse function, Err(_) means catch all types of errors
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid nuumber!!");
                continue;
            }
        }; // we are shadowing old guess with new one of type integer, this ensures re-use of a variable, trim function here trims out any whitespace, the parse method parses the string to a number
        println!("You guessed: {}", guess); 
        match guess.cmp(&secret_number) { // match expression consists of two parts, pattern and the code to run if matched to a pattern
            Ordering::Less => {
                count += 1; // count incremented when guessed wrong
                println!("Too small!");
            } 
            Ordering::Greater => {
                count += 1; // count incremented when guessed wrong
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                println!("It took you {} tries to guess", count);
                break; // breaking if the user guesses the number correctly and it ends the infinite loop
            }
        }
    }
}
