use std::io;  // Equivalent to using namespace std; in c++

fn main() {
    println!("I\'m thinking of a number between 1 and 10!");
    let mut guess = String::new();  // new() is (an) assoc-func.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
}
