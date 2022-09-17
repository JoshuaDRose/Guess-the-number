use rand::Rng;
use std::io;  // Equivalent to using namespace std; in c++

fn main() {
    println!("I\'m thinking of a number between 1 and 10!");
    // Instance type string
    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1, 10);
    // Receive user input
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // Print out guess
    println!("You guessed: {}", guess);
}
