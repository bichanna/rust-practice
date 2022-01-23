use std::io;

fn main() {
    println!("This is guessing game program!");
    let mut guess = String::new();
    println!("Guess a number");
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guesses: {}", guess);
}
