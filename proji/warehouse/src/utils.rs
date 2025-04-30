/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       utils.rs
 * Purpose:    Contains helper functions used across the code.
 * =======================================================================
 */

 use std::io::{self, Write};
 use std::time::{SystemTime, UNIX_EPOCH};
 use chrono::{NaiveDate, Utc};
 
 // Reads user input from the terminal with a prompt
 pub fn read_input(text: &str) -> String {
     print!("{}", text);
     io::stdout()
        .flush()
        .unwrap();
 
     let mut input = String::new();
     io::stdin()
        .read_line(&mut input)
        .unwrap();
     input.trim().to_string()
 }
 
 // stop when is a valid usize
 pub fn parse_usize_input(text: &str) -> usize {
     loop {
         let input = read_input(text);
         match input.parse::<usize>() {
             Ok(value) => return value,
             Err(_) => println!("Please enter a valid number."),
         }
     }
 }
 
 // checks expiry date 
 pub fn is_expired(expiry_date: NaiveDate) -> bool {
     let today = Utc::now().naive_utc().date();
     expiry_date < today
 }
 
