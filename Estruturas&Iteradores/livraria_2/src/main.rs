/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Library implementation
 *              + find a book through tittle, isbn, author
 *              + find through keyword (intersection and union)
 * =======================================================================
 */

use std::collections::HashMap;
use std::io::{self, Write};

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
    }

    fn remove_book(&mut self, isbn: &str) -> Option<Book> {
        self.books.remove(isbn)
    }

    fn request_book(&mut self, isbn: &str) -> Option<Book> {
        if let Some(book) = self.books.get_mut(isbn) {
            if book.quantity > 0 {
                book.quantity -= 1;
                Some(book.clone())
            } else {
                println!("Sorry, this book is out of stock.");
                None
            }
        } else {
            println!("Book not found.");
            None
        }
    }

    fn return_book(&mut self, isbn: &str) {
        if let Some(book) = self.books.get_mut(isbn) {
            book.quantity += 1;
        } else {
            println!("Sorry, book not found.");
        }
    }

    fn find_by_title(&self, title: &str) {
        for book in self.books.values() {
            if book.title.eq_ignore_ascii_case(title) {
                println!(
                    "Found: '{}' by {} (ISBN: {})",
                    book.title, book.author, book.isbn
                );
            }
        }
    }

    fn find_by_author(&self, author: &str) {
        for book in self.books.values() {
            if book.author.eq_ignore_ascii_case(author) {
                println!("Found: '{}' (ISBN: {})", book.title, book.isbn);
            }
        }
    }

    fn search_by_keywords_union(&self, keywords: Vec<&str>) {
        for book in self.books.values() {
            if keywords
                .iter()
                .any(|k| book.keywords.contains(&k.to_string()))
            {
                println!(
                    "MATCH (union): '{}' by {} (ISBN: {})",
                    book.title, book.author, book.isbn
                );
            }
        }
    }

    fn search_by_keywords_intersection(&self, keywords: Vec<&str>) {
        for book in self.books.values() {
            if keywords
                .iter()
                .all(|k| book.keywords.contains(&k.to_string()))
            {
                println!(
                    "MATCH (intersection): '{}' by {} (ISBN: {})",
                    book.title, book.author, book.isbn
                );
            }
        }
    }
}

fn main() {
    let mut bookstore = Bookstore::new();

    bookstore.add_book(Book {
        isbn: "978-0999999999".to_string(),
        title: "History of the Roman Empire".to_string(),
        author: "Will Smith".to_string(),
        keywords: vec!["Roman".to_string(), "History".to_string()],
        quantity: 9,
    });

    bookstore.add_book(Book {
        isbn: "978-020000224".to_string(),
        title: "The Programmer".to_string(),
        author: "Emma Stone".to_string(),
        keywords: vec!["programming".to_string(), "Rust".to_string()],
        quantity: 26,
    });

    bookstore.add_book(Book {
        isbn: "088-06743599".to_string(),
        title: "Marketing".to_string(),
        author: "Roberto".to_string(),
        keywords: vec![
            "moneymaker".to_string(),
            "marketing".to_string(),
            "business".to_string(),
        ],
        quantity: 1,
    });

    loop {
        println!(
            "
 --- Bookstore Menu ---"
        );
        println!("1: Add a book");
        println!("2: Remove a book");
        println!("3: Request a book");
        println!("4: Return a book");
        println!("5: Search by single keyword");
        println!("6: Search by title");
        println!("7: Search by author");
        println!("8: Search by keywords (union)");
        println!("9: Search by keywords (intersection)");
        println!("10: Exit");
        print!("Please enter an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        let option = option.trim();

        match option {
            "1" => {
                let isbn = input("Enter ISBN:");
                let title = input("Enter title:");
                let author = input("Enter author:");
                let keywords_str = input("Enter keywords separated by '&':");
                let quantity = input("Enter quantity:").parse::<u32>().unwrap_or(0);

                let keywords = keywords_str
                    .split('&')
                    .map(|s| s.trim().to_string())
                    .collect();

                bookstore.add_book(Book {
                    isbn,
                    title,
                    author,
                    keywords,
                    quantity,
                });

                println!("Book added successfully.");
            }
            "2" => {
                let isbn = input("Enter ISBN of the book to remove:");
                if bookstore.remove_book(&isbn).is_some() {
                    println!("Book removed successfully.");
                } else {
                    println!("Book not found.");
                }
            }
            "3" => {
                let isbn = input("Enter ISBN of the book to request:");
                if bookstore.request_book(&isbn).is_some() {
                    println!("Book requested successfully.");
                }
            }
            "4" => {
                let isbn = input("Enter ISBN of the book to return:");
                bookstore.return_book(&isbn);
                println!("Book returned successfully.");
            }
            "5" => {
                let keyword = input("Enter a keyword to search for:");
                bookstore.search_by_keywords_union(vec![&keyword]);
            }
            "6" => {
                let title = input("Enter title:");
                bookstore.find_by_title(&title);
            }
            "7" => {
                let author = input("Enter author:");
                bookstore.find_by_author(&author);
            }
            "8" => {
                let keywords = input("Enter keywords (union) separated by '&':");
                let terms: Vec<&str> = keywords.split('&').map(|s| s.trim()).collect();
                bookstore.search_by_keywords_union(terms);
            }
            "9" => {
                let keywords = input("Enter keywords (intersection) separated by '&':");
                let terms: Vec<&str> = keywords.split('&').map(|s| s.trim()).collect();
                bookstore.search_by_keywords_intersection(terms);
            }
            "10" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option."),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
