use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1, 10);

    println!("I\'m thinking of a number between 1 and 10!");
    println!("The secret number is: {}", secret_num);

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number");

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Higher"),
        Ordering::Higher => println!("Lower"),
        Ordering::Equal => println!("Correct!"),
    }
}
