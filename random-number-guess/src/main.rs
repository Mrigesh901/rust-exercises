use rand:: Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
// rng trait includes method for generating random numbers

    println!("Guess the number");
    let mut turns : u32 = 0;
    loop {
        println!("Please input your guess ");
        turns+=1;
        let mut guess = String::new();
        
        // read the input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Convert string input to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is {}", guess);

        // Check if guess is correct

        if guess < secret_number{
            println!("Too small");
        }
        else if guess > secret_number {
            println!("Too big");
        }
        else {
            println!("You guessed it right!!, you took {} chances to get it correct", turns);
            break;
        }
    }
}
