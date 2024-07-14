use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
    
        println!("Please enter your guess..");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is lesser than the secret number!"),
            Ordering::Greater => println!("Your guess is greater than the secret number!"),
            Ordering::Equal => {
                println!("You win!\nYour guess is equal to the secret number..");
                break;
            },
        };
    }
}
