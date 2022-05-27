use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1 .. 101);

    println!("The not so secret number has been generated as: {}", secret_number);

    let mut guess_count: i32 = 0;
    let mut correct_guess = false;
    loop {
        println!("Please enter a number between 1 and 100: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read user input");

        let guess: u32 = guess.trim().parse().expect("Please enter a number!");

        println!("You guessed {}", guess);
        guess_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less =>
                println!("Too small!"),
            Ordering::Greater =>
                println!("Too big!"),
            Ordering::Equal => {
                println!("Good Guess! You got it");
                correct_guess = true;
                break;
            }
            }
        }

        println!("You guessed the answer in {} turns", guess_count);
}
