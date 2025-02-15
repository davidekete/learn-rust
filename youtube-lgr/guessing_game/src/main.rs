use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess");
        println!("{}", &secret_number);

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                println!("You guessed: {}", guess);

                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("{}", "Too small".red()),
                    Ordering::Equal => {
                        println!("{}", "You guessed the right number".green());
                        break;
                    }
                    Ordering::Greater => println!("{}", "Too big".red()),
                }
            }
            Err(e) => println!("failed to read line {}", e),
        }
    }
}
