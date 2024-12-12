use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_num: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Take a guess from 1-100 (inclusive): ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed IO");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please only enter a valid integer!");
                continue;
            }
        };

        if let 1..=100 = guess {
            match guess.cmp(&rand_num) {
                Ordering::Less => println!("Too low!"),
                Ordering::Greater => println!("Too high!"),
                _ => {
                    println!("You won!");
                    break;
                }
            }
        } else {
            println!("Input must be from 1-100 (inclusive)");
            continue;
            // std::process::exit;
        }
    }
}
