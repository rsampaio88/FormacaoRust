/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Implementation of a library
 *              - add, remove, borrow and return books system
 * =======================================================================
 */

use std::collections::HashMap;
use std::io::{self, Write};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    keywords: Vec<String>,
    quantity: u32,
}

struct Bookstore {
    books: HashMap<String, Book>,
}

impl Bookstore {
    fn new() -> Self {
        Bookstore {
            books: HashMap::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn.clone(), book);
        println!("Book added, thank you.");
    }

    fn remove_book(&mut self, isbn: &str) {
        if self.books.remove(isbn).is_some() {
            println!("Book removed, thank you.");
        } else {
            println!("Sorry, book not found.");
        }
    }

    fn request_book(&mut self, isbn: &str) {
        match self.books.get_mut(isbn) {
            Some(book) if book.quantity > 0 => {
                book.quantity -= 1;
                println!("Book requested, thank you.");
            }
            Some(_) => println!("Sorry, book is out of stock."),
            None => println!("Sorry, book not found."),
        }
    }

    fn return_book(&mut self, isbn: &str) {
        match self.books.get_mut(isbn) {
            Some(book) => {
                book.quantity += 1;
                println!("Book returned, thank you.");
            }
            None => println!("Sorry, book not found."),
        }
    }
}

fn main() {
    let mut bookstore = Bookstore::new();

    loop {
        println!("\n--- Bookstore Menu ---");
        println!("1: Add a book");
        println!("2: Remove a book");
        println!("3: Request a book");
        println!("4: Return a book");
        println!("5: Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                let book = input_book();
                bookstore.add_book(book);
            }
            "2" => {
                let isbn = input("Enter ISBN of the book to remove:");
                bookstore.remove_book(&isbn);
            }
            "3" => {
                let isbn = input("Enter ISBN of the book to request:");
                bookstore.request_book(&isbn);
            }
            "4" => {
                let isbn = input("Enter ISBN of the book to return:");
                bookstore.return_book(&isbn);
            }
            "5" => {
                println!("See you!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn input_book() -> Book {
    let isbn = input("Enter ISBN:");
    let title = input("Enter title:");
    let author = input("Enter author:");
    let keywords_input = input("Enter keywords separated by '&':");
    let quantity = input("Enter quantity:").parse::<u32>().unwrap_or(0);

    let keywords = keywords_input
        .split('&')
        .map(|s| s.trim().to_string())
        .collect();

    Book {
        isbn,
        title,
        author,
        keywords,
        quantity,
    }
}

fn input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
