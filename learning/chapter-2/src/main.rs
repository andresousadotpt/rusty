use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); // Range expression is like start..=end

    loop {
        println!("Guess a number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // Pass a reference of guess
            // references are immutable by default that is why we use &mut so we can change the string content
            .expect("Failed to read line"); // Potential error on result, this handles it.

        // We can shadow an existing variable
        // So we can reuse the previous variable name we will talk more about thus on Chapter 3.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        // This is pattern matching.
        // guess.cmp() returns a Ordering, which we could verify if the Ordering is Ordering::less, etc...
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        };
    }

    println!("The secret was {}", secret_number);
}
