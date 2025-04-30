/*
 * ==========================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    The user inputs a number in order to guess the secret number
 * ==========================================================================
 */

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1..101);
    println!("Guess the number between 1 and 100!");

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ups :(, failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guesses : {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Congratulations, you guessed!");
                break;
            }
        }
    }
}
