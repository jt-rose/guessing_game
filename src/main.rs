use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1 .. 101);

    println!("The not so secret number has been generated as: {}", secret_number);

    println!("Please enter a number between 1 and 100: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read user input");

    println!("You guessed {}", guess);
}
