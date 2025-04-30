/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    interactive string manipulation: insert, remove, case
 *            change, occurrence check, and repeat by alphabet position.
 * =======================================================================
 */

/*
Notes:

    .c.is_ascii_alphabetic(): only alpha
    .trim(): removes whitespaces, \n on the begining and end
    .chars(): splits the string in chars "abcd" -> 'a' 'b' 'c' 'd'
    .cycle(): creates an iterator to repeat char a,b,c,a,b,c,a,b,c,..
    as u8: converts 'a' to ascii value
    .to_string: convert to string
    .repeat(n): repeates n times
*/

use std::io;

fn main() {
    loop {
        println!("\nChoose an option:");
        println!("1: Add something to a string.");
        println!("2: Remove something from a string.");
        println!("3: Lowercase the string.");
        println!("4: Uppercase the string.");
        println!("5: Occurrence in string.");
        println!("6: Repeat letter according to order in the alphabet.");
        println!("0: Stop.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try a number.");
                continue;
            }
        };

        match input {
            0 => {
                println!("Exiting.");
                break;
            }
            1 => add_string(),
            2 => {
                let result = remove_string();
                println!("Final String: {}", result);
            }
            3 => {
                println!("Enter the string:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Error");
                println!("Final String: {}", input.trim().to_lowercase());
            }
            4 => {
                println!("Enter the string:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Error");
                println!("Final String: {}", input.trim().to_uppercase());
            }
            5 => occur_string(),
            6 => {
                let result = repeat_string();
                println!("Repeated string: {}", result);
            }
            _ => println!("Invalid option, try again."),
        }
    }
}

fn add_string() {
    println!("Enter a string:");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Error");
    let str = str.trim();

    println!("Enter what you want to add:");
    let mut add = String::new();
    io::stdin().read_line(&mut add).expect("Error");
    let add = add.trim();

    println!("Enter the index where you want to insert:");
    let mut index_str = String::new();
    io::stdin().read_line(&mut index_str).expect("Error");

    let index: usize = match index_str.trim().parse() {
        Ok(i) if i <= str.len() => i,
        _ => {
            println!("Invalid index.");
            return;
        }
    };

    let result = format!("{}{}{}", &str[..index], add, &str[index..]);
    println!("Final string: {}", result);
}

fn remove_string() -> String {
    println!("Enter a string:");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Error");

    println!("Enter what you want to remove:");
    let mut remove = String::new();
    io::stdin().read_line(&mut remove).expect("Error");

    str.trim().replace(remove.trim(), "")
}

fn occur_string() {
    println!("Enter a string:");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Error");

    println!("Enter what you want to search for:");
    let mut sub = String::new();
    io::stdin().read_line(&mut sub).expect("Error");

    println!("1: Count how many times it appears.");
    println!("2: Check if it appears.");
    println!("0: Cancel.");

    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Error");

    let str = str.trim();
    let sub = sub.trim();

    match op.trim() {
        "1" => {
            let count = str.matches(sub).count();
            println!("It occurs {} time(s).", count);
        }
        "2" => {
            if str.contains(sub) {
                println!("It's in the string.");
            } else {
                println!("It's not in the string.");
            }
        }
        "0" => println!("Cancelled."),
        _ => println!("Invalid option."),
    }
}

fn repeat_string() -> String {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    input
        .trim()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| {
            let position = (c.to_ascii_lowercase() as u8 - 96) as usize;
            c.to_string().repeat(position)
        })
        .collect()
}
