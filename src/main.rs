extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }   

        }
    }    
}


fn main() {
       
    loop {
        guessing_game();

        loop {

            println!("Do you want to play again? (y/n)");

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");

            let user_input = user_input.trim();

            match user_input.to_lowercase().as_str() {
                "y" => {
                    println!("Starting a new game...");
                    break;
                },
                "n" => {
                    println!("Thanks for playing! Goodbye.");
                    return;
                },
                _ => {
                println!("Invalid input, please type 'y' or 'n'.");
                },
            }
        
        }

    }
} 


