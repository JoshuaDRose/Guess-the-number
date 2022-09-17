use rand::Rng;
use std::cmp::Ordering;
use std::io;  // Equivalent to using namespace std; in c++

fn main() {

    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1, 10);

    println!("I\'m thinking of a number between 1 and 10!");

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
