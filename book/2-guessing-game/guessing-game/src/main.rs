// remember: this is the file that runs all the main stuff

use rand::Rng; // random number generator within a range
use std::cmp::Ordering; // allows for comparison between values
use std::io; // allows for I/O // random number generator

fn main() {
  // main function, everything happens here
  // prints a nice box to the screen
  print!("|-----------------------|\n|     GUESSING GAME     |\n|-----------------------|\n");
  println!("Guess a number from 1-10.");

  // generate a number to guess (from 1-11, 11 exclusive)
  let secret_number = rand::thread_rng().gen_range(1..11);

  loop {
    // infinite loop - let the user keep guessing
    println!("Enter your guess.");

    // create guess (mutable variable)
    let mut guess = String::new(); // ::new() is an associated function of the String datatype

    // read the input into the guess variable
    // otherwise, if input is unreadable, print "Failed to read line!" (handling errors)
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line!");

    // convert guess to an Int (reuse the guess variable but assign it to a new value)
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue, // continue if received an error
    };

    // rust's version of placeholder strings, using {} like python
    println!("You guessed: {}", guess);

    // rust's version of the switch statement
    // make sure guess is a int!
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
