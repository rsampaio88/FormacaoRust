/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Final implemention of library.
 *             +info details on audiobooks, paintings and statues.
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

#[derive(Debug, Clone)]
struct Audiobook {
    isbn: String,
    title: String,
    author: String,
    keywords: Vec<String>,
    quantity: u32,
    duration: u32,
}

#[derive(Debug, Clone)]
struct Statue {
    isbn: String,
    title: String,
    author: String,
    keywords: Vec<String>,
    quantity: u32,
    dimensions: (f32, f32, f32),
}

#[derive(Debug, Clone)]
struct Painting {
    isbn: String,
    title: String,
    author: String,
    keywords: Vec<String>,
    quantity: u32,
    dimensions: (f32, f32),
}

#[derive(Debug, Clone)]
enum ItemType {
    Book(Book),
    Audiobook(Audiobook),
    Statue(Statue),
    Painting(Painting),
}

struct Bookstore {
    items: HashMap<String, ItemType>,
}

impl Bookstore {
    fn new() -> Self {
        Bookstore {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: ItemType) {
        let isbn = match &item {
            ItemType::Book(b) => &b.isbn,
            ItemType::Audiobook(a) => &a.isbn,
            ItemType::Statue(s) => &s.isbn,
            ItemType::Painting(p) => &p.isbn,
        };
        self.items.insert(isbn.clone(), item);
    }

    fn remove_item(&mut self, isbn: &str) -> Option<ItemType> {
        self.items.remove(isbn)
    }

    fn request_item(&mut self, isbn: &str) -> Option<ItemType> {
        if let Some(item) = self.items.get_mut(isbn) {
            match item {
                ItemType::Book(b) if b.quantity > 0 => {
                    b.quantity -= 1;
                    Some(item.clone())
                }
                ItemType::Audiobook(a) if a.quantity > 0 => {
                    a.quantity -= 1;
                    Some(item.clone())
                }
                ItemType::Statue(s) if s.quantity > 0 => {
                    s.quantity -= 1;
                    Some(item.clone())
                }
                ItemType::Painting(p) if p.quantity > 0 => {
                    p.quantity -= 1;
                    Some(item.clone())
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn return_item(&mut self, isbn: &str) {
        if let Some(item) = self.items.get_mut(isbn) {
            match item {
                ItemType::Book(b) => b.quantity += 1,
                ItemType::Audiobook(a) => a.quantity += 1,
                ItemType::Statue(s) => s.quantity += 1,
                ItemType::Painting(p) => p.quantity += 1,
            }
        }
    }

    fn get_duration(&self, isbn: &str) -> Result<u32, String> {
        self.items
            .get(isbn)
            .and_then(|item| {
                if let ItemType::Audiobook(a) = item {
                    Some(Ok(a.duration))
                } else {
                    Some(Err("Not an audiobook.".to_string()))
                }
            })
            .unwrap_or_else(|| Err("Item not found.".to_string()))
    }

    fn get_dimensions(&self, isbn: &str) -> Result<String, String> {
        self.items
            .get(isbn)
            .and_then(|item| match item {
                ItemType::Statue(s) => Some(Ok(format!("{:?}", s.dimensions))),
                ItemType::Painting(p) => Some(Ok(format!("{:?}", p.dimensions))),
                _ => Some(Err("No dimensions available.".to_string())),
            })
            .unwrap_or_else(|| Err("Item not found.".to_string()))
    }

    fn find_by_title(&self, title: &str) -> Option<&ItemType> {
        self.items.values().find(|item| match item {
            ItemType::Book(b) => b.title == title,
            ItemType::Audiobook(a) => a.title == title,
            ItemType::Statue(s) => s.title == title,
            ItemType::Painting(p) => p.title == title,
        })
    }

    fn find_by_author(&self, author: &str) -> Option<&ItemType> {
        self.items.values().find(|item| match item {
            ItemType::Book(b) => b.author == author,
            ItemType::Audiobook(a) => a.author == author,
            ItemType::Statue(s) => s.author == author,
            ItemType::Painting(p) => p.author == author,
        })
    }
}

fn input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let mut bookstore = Bookstore::new();

    loop {
        println!("\n--- Bookstore Menu ---");
        println!("1. Add item");
        println!("2. Remove item");
        println!("3. Request item");
        println!("4. Return item");
        println!("5. Search by title");
        println!("6. Search by author");
        println!("7. Get duration");
        println!("8. Get dimensions");
        println!("9. Exit");

        match input("Choose:").as_str() {
            "1" => {
                let kind = input("1: Book, 2: Audiobook, 3: Statue, 4: Painting:");
                let isbn = input("ISBN:");
                let title = input("Title:");
                let author = input("Author:");
                let keywords: Vec<String> = input("Keywords separated by &:")
                    .split('&')
                    .map(|s| s.trim().to_string())
                    .collect();
                let quantity = input("Quantity:").parse().unwrap_or(0);

                let item = match kind.as_str() {
                    "1" => ItemType::Book(Book {
                        isbn,
                        title,
                        author,
                        keywords,
                        quantity,
                    }),
                    "2" => {
                        let duration = input("Duration (minutes):").parse().unwrap_or(0);
                        ItemType::Audiobook(Audiobook {
                            isbn,
                            title,
                            author,
                            keywords,
                            quantity,
                            duration,
                        })
                    }
                    "3" => {
                        let dims: Vec<f32> = input("Dimensions (h w d):")
                            .split_whitespace()
                            .filter_map(|v| v.parse().ok())
                            .collect();
                        ItemType::Statue(Statue {
                            isbn,
                            title,
                            author,
                            keywords,
                            quantity,
                            dimensions: (dims[0], dims[1], dims[2]),
                        })
                    }
                    "4" => {
                        let dims: Vec<f32> = input("Dimensions (h w):")
                            .split_whitespace()
                            .filter_map(|v| v.parse().ok())
                            .collect();
                        ItemType::Painting(Painting {
                            isbn,
                            title,
                            author,
                            keywords,
                            quantity,
                            dimensions: (dims[0], dims[1]),
                        })
                    }
                    _ => {
                        println!("Invalid type.");
                        continue;
                    }
                };

                bookstore.add_item(item);
                println!("Item added.");
            }
            "2" => {
                let isbn = input("ISBN to remove:");
                bookstore.remove_item(&isbn);
            }
            "3" => {
                let isbn = input("ISBN to request:");
                if bookstore.request_item(&isbn).is_some() {
                    println!("Item requested.");
                } else {
                    println!("Item not available.");
                }
            }
            "4" => {
                let isbn = input("ISBN to return:");
                bookstore.return_item(&isbn);
                println!("Returned.");
            }
            "5" => {
                let title = input("Title:");
                match bookstore.find_by_title(&title) {
                    Some(i) => println!("Found: {:?}", i),
                    None => println!("Not found."),
                }
            }
            "6" => {
                let author = input("Author:");
                match bookstore.find_by_author(&author) {
                    Some(i) => println!("Found: {:?}", i),
                    None => println!("Not found."),
                }
            }
            "7" => {
                let isbn = input("ISBN:");
                match bookstore.get_duration(&isbn) {
                    Ok(d) => println!("Duration: {} minutes", d),
                    Err(e) => println!("{}", e),
                }
            }
            "8" => {
                let isbn = input("ISBN:");
                match bookstore.get_dimensions(&isbn) {
                    Ok(d) => println!("Dimensions: {}", d),
                    Err(e) => println!("{}", e),
                }
            }
            "9" => break,
            _ => println!("Invalid option."),
        }
    }
}
