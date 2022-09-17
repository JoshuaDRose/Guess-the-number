use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    let range: u32 = 101;
    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1, range);

    println!("Guess a number from 1 to 100!");
    println!("The secret number is: {}", secret_num);

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number");

    loop {
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Higher"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal => println!("Correct!"),
        }
}
