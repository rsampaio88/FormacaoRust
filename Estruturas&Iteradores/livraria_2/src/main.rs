
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
    fn new() -> Bookstore {
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
            println!("Book not found.");
        }
    }

    fn find_books_by_keyword(&self, keyword: &str) {
        let mut found_books = false;
        for book in self.books.values() {
            if book.keywords.contains(&keyword.to_string()) {
                println!(
                    "Found book: '{}' by {} (ISBN: {})",
                    book.title, book.author, book.isbn
                );
                found_books = true;
            }
        }

        if !found_books {
            println!("No books found with the keyword '{}'", keyword);
        }
    }
}

fn main() {
    let mut bookstore = Bookstore::new();

    bookstore.add_book(Book {
        isbn: "978-0999999999".to_string(),
        title: " History of the Roman Empire".to_string(),
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
        println!("\n--- Bookstore Menu ---");
        println!("1: Add a book");
        println!("2: Remove a book");
        println!("3: Request a book");
        println!("4: Return a book");
        println!("5: Search for books by keyword");
        println!("6: Leave");
        print!("Please enter an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        let option = option.trim();

        match option {
            "1" => {
                let mut isbn = String::new();
                let mut title = String::new();
                let mut author = String::new();
                let mut keywords = String::new();
                let mut quantity = String::new();

                println!("Enter ISBN: ");
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read line");
                println!("Enter title: ");
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");
                println!("Enter author: ");
                io::stdin()
                    .read_line(&mut author)
                    .expect("Failed to read line");
                println!("Enter keywords separated by '&': ");
                io::stdin()
                    .read_line(&mut keywords)
                    .expect("Failed to read line");
                println!("Enter quantity: ");
                io::stdin()
                    .read_line(&mut quantity)
                    .expect("Failed to read line");

                let isbn = isbn.trim().to_string();
                let title = title.trim().to_string();
                let author = author.trim().to_string();
                let keywords: Vec<String> = keywords
                    .trim()
                    .split('&')
                    .map(|s| s.trim().to_string())
                    .collect();
                let quantity: u32 = quantity.trim().parse().unwrap_or(0);

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
                let mut isbn = String::new();
                println!("Enter ISBN of the book to remove: ");
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read line");

                let isbn = isbn.trim();
                if bookstore.remove_book(isbn).is_some() {
                    println!("Book removed successfully.");
                } else {
                    println!("Book not found.");
                }
            }
            "3" => {
                let mut isbn = String::new();
                println!("Enter ISBN of the book to request: ");
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read line");

                let isbn = isbn.trim();
                if bookstore.request_book(isbn).is_some() {
                    println!("Book requested successfully.");
                }
            }
            "4" => {
                let mut isbn = String::new();
                println!("Enter ISBN of the book to return: ");
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read line");

                let isbn = isbn.trim();
                bookstore.return_book(isbn);
                println!("Book returned successfully.");
            }
            "5" => {
                let mut keyword = String::new();
                println!("Enter a keyword to search for: ");
                io::stdin()
                    .read_line(&mut keyword)
                    .expect("Failed to read line");

                let keyword = keyword.trim().to_string();
                bookstore.find_books_by_keyword(&keyword);
            }
            "6" => {
                println!("See you soon!");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
