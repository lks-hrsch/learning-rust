use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let low = 0;
    let high = 10;

    loop {
        println!("Please input your guess. Between {} - {}", low, high);

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(low..high);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("Expected a number! - Redo");
                continue;
            },
        };

        println!("The secret number is {}", secret_number);
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
                
        }

        println!("---------------------------------------------------");
    }
}