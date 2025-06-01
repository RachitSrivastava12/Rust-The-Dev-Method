use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
   

    loop {
        let mut guess = String::new();
        println!("enter you guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("the guess is {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("the number guessed is less"),
            Ordering::Greater => println!("the number guessed is greater"),
            Ordering::Equal => {
                println!("your guess is correct");
                break;
            }
        }
    }
}
