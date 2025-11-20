use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("Corect Num: {}", secret_number);
    let name = "Abdelaziz"; //binding
    println!("Hey guess a number ");
    loop{
        let mut num = String::new(); // mut to change the variable as it is not mutable by defualt
        io::stdin().read_line(&mut num).expect("Error reading input");
        // let num: u32 = num.trim().parse().expect("Error with parsing");
        let num: u32 = match num.trim().parse() {
            Ok (guess)=> guess,
            Err(e) => {
                println!("Error with parsing, try again {e}");
                continue;
            }
        };
        // if secret_number > num {
        //     println!("You guessing was too low");
        // } else if secret_number < num {
        //     println!("You guessing was too high");
        // } else {
        //     println!("You guessing was correct");
        // }
        // println!("You guessed {}", num);

        // let mut message = if num > secret_number {
        //     String::from("You guessed too high")
        // } else if num < secret_number {
        //     String::from("You guessed too low")
        // } else {
        //     String::from("You guessed CORRECT")
        // };
        // println!("{message}");

        match num.cmp(&secret_number) {
            Ordering::Greater => println!("You guessed too high"),
        
            Ordering::Less => println!("You guessed too low"),
        
            Ordering::Equal => { 
                println!("You guessed CORRECT.");
                break;
            }
        };
    }
}
