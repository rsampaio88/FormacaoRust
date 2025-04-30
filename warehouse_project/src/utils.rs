/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       utils.rs
 * Purpose:    Utility functions for console input and number parsing
 *             with validation.
 * =======================================================================
 */

use std::io::{self, Write};

pub fn read_input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn parse_input(text: &str) -> usize {
    loop {
        let input = read_input(text);
        match input.parse::<usize>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
