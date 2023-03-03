use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn randomise() -> u16 {
    println!("Randomising key number...");
    let secret_number = rand::thread_rng().gen_range(0..10);

    return secret_number;
}

fn main() {
    println!("Welcome to Rust number guessing game!");

    let secret = randomise();
    let mut guess = String::new();

    println!("Guess a number:");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input...");

    let guess_int: u16 = guess.trim().parse().expect("Wrong format...Please enter a number");

    println!("Secret key: {}", secret);
    println!("You guessed: {}", guess);

    match guess_int.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Matched! You win!")
    };
}