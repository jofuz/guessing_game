// input/output library
use std::io;

// randomizer library
use rand::Rng;

fn main() {
    // print line
    println!("Guess the number!");
    // thread_rng() - thread range uses a single thread when empty
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    
    println!("Please input your guess.");
    // mut = mutable variable
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}