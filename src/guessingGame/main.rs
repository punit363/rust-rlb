use std::io;
use std::cmp::Ordering;
use rand::RngExt;
use colored::*;

fn main() {
    println!("Please input your guess!");

    let secret_number = rand::rng().random_range(1..=100);
    
    
    println!("The secret number is {}",secret_number);
    
    loop{
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read line");
        
        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("Enter a valid integer");
                continue;
            }
        };
        
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("{}","Oops! Your guess is low!".red()),
            Ordering::Equal=>{println!("{}","Congratulations! Your guess was correct!".green());                
            break;
        },
            Ordering::Greater=>println!("{}","Oops! Your guess is high!".red())
        }
        
        println!("Guess: {}",guess)
    }
}
