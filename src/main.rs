use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input a guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        print!("You guessed {guess}. ");

        match  guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You found the number!");
                break;
            },
            Ordering::Greater => println!("Your guess is too large!"),
            Ordering::Less => println!("Your guess is too small!")
        }
    }
}
