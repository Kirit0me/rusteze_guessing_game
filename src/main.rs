use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=1000);

    loop{
    println!("Guess the number \nPlease input your number :\n");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed : {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        match guess.cmp(&secret){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You W!");
                break;
                
            }
        }
    }
}
