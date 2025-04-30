use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("*ssh* the secret number is {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        let byte_count = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // I do not like this data type redefinition shadowing business; that's what we call a compilation error in most languages, but in Rust it's a feature? This is what comes of trusting a crab with programming language design. It already bit me when I chose not to move the guess declaration inside the loop because why would I need to? Well, if we redefine him to a u32 that would do it... Don't think I'll be using this lang feature.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {e}. Your guess must be a number.");
                continue;
            }
        };
        println!(
            "You guessed: {}, which contains {} bytes.",
            guess, byte_count
        );
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Bigger!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
        }
    }
}
