use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let mut how_many = String::new();
    print!("How many random numbers you want to guess?");
    io::stdin().read_line(&mut how_many).expect("Error readin input");

    // let secret_number = rand::thread_rng().gen_range(1..=10);
    let num_gueses: u8 = how_many.trim().parse().expect("Error parsing input");
    let mut secret_numbers = Vec::new();
    for _ in 0..num_gueses{
        secret_numbers.push(rand::thread_rng().gen_range(1..=10));
    }

    print!("{secret_numbers:?}");
    let mut guesses_made = 0;

    println!("Hey guess a number ");
    while guesses_made < num_gueses{
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
        match num.cmp(&secret_numbers[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high"),
        
            Ordering::Less => println!("You guessed too low"),
        
            Ordering::Equal => { 
                println!("You guessed CORRECT.");
                guesses_made+=1;
                if guesses_made < num_gueses {
                    println!("Let's now try the next number");
                }
            }
        };
    };
    /*
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
    */
}
