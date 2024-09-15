use rand::Rng;
use std::io;

fn main() {
    let mut guess = String::new();

    let secret_number: i8 = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {}", secret_number);

    println!("Please enter a number from 1 to 100");
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    let guess: i8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if secret_number == guess {
        println!("You guessed it right!");
    }
}
