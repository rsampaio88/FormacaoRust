/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Library 2 implementation
 *              + audiobooks, paintings and statues
 * =======================================================================
 */

use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
enum ItemType {
    Book,
    Audiobook,
    Statue,
    Painting,
}

#[derive(Debug, Clone)]
struct Item {
    isbn: String,
    title: String,
    author: String,
    keywords: Vec<String>,
    quantity: u32,
    item_type: ItemType,
}

struct Bookstore {
    items: HashMap<String, Item>,
}

impl Bookstore {
    fn new() -> Bookstore {
        Bookstore {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.insert(item.isbn.clone(), item);
    }

    fn remove_item(&mut self, isbn: &str) -> Option<Item> {
        self.items.remove(isbn)
    }

    fn request_item(&mut self, isbn: &str) -> Option<Item> {
        if let Some(item) = self.items.get_mut(isbn) {
            if item.quantity > 0 {
                item.quantity -= 1;
                Some(item.clone())
            } else {
                println!("Sorry, this item is out of stock.");
                None
            }
        } else {
            println!("Item not found.");
            None
        }
    }

    fn return_item(&mut self, isbn: &str) {
        if let Some(item) = self.items.get_mut(isbn) {
            item.quantity += 1;
        } else {
            println!("Sorry, item not found.");
        }
    }

    fn find_by_title(&self, title: &str) {
        for item in self.items.values() {
            if item.title.eq_ignore_ascii_case(title) {
                println!(
                    "Found: '{}' by {} (ISBN: {})",
                    item.title, item.author, item.isbn
                );
            }
        }
    }

    fn find_by_author(&self, author: &str) {
        for item in self.items.values() {
            if item.author.eq_ignore_ascii_case(author) {
                println!("Found: '{}' (ISBN: {})", item.title, item.isbn);
            }
        }
    }

    fn search_by_keywords_union(&self, keywords: Vec<&str>) {
        for item in self.items.values() {
            if keywords
                .iter()
                .any(|k| item.keywords.contains(&k.to_string()))
            {
                println!(
                    "MATCH (union): '{}' by {} (ISBN: {})",
                    item.title, item.author, item.isbn
                );
            }
        }
    }

    fn search_by_keywords_intersection(&self, keywords: Vec<&str>) {
        for item in self.items.values() {
            if keywords
                .iter()
                .all(|k| item.keywords.contains(&k.to_string()))
            {
                println!(
                    "MATCH (intersection): '{}' by {} (ISBN: {})",
                    item.title, item.author, item.isbn
                );
            }
        }
    }
}

fn main() {
    let mut bookstore = Bookstore::new();

    bookstore.add_item(Item {
        isbn: "978-0999999999".to_string(),
        title: "History of the Roman Empire".to_string(),
        author: "Will Smith".to_string(),
        keywords: vec!["Roman".to_string(), "History".to_string()],
        quantity: 9,
        item_type: ItemType::Audiobook,
    });

    bookstore.add_item(Item {
        isbn: "978-020000224".to_string(),
        title: "The Programmer".to_string(),
        author: "Emma Stone".to_string(),
        keywords: vec!["programming".to_string(), "Rust".to_string()],
        quantity: 26,
        item_type: ItemType::Book,
    });

    bookstore.add_item(Item {
        isbn: "088-06743599".to_string(),
        title: "Marketing".to_string(),
        author: "Roberto".to_string(),
        keywords: vec![
            "moneymaker".to_string(),
            "marketing".to_string(),
            "business".to_string(),
        ],
        quantity: 1,
        item_type: ItemType::Painting,
    });

    loop {
        println!(
            "
--- Bookstore Menu ---"
        );
        println!("1: Add an item");
        println!("2: Remove an item");
        println!("3: Request an item");
        println!("4: Return an item");
        println!("5: Search for items by single keyword");
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
                let type_input =
                    input("Enter item type (1: Book, 2: Audiobook, 3: Statue, 4: Painting):");

                let keywords = keywords_str
                    .split('&')
                    .map(|s| s.trim().to_string())
                    .collect();
                let item_type = match type_input.trim() {
                    "1" => ItemType::Book,
                    "2" => ItemType::Audiobook,
                    "3" => ItemType::Statue,
                    "4" => ItemType::Painting,
                    _ => {
                        println!("Invalid type, defaulting to Book.");
                        ItemType::Book
                    }
                };

                bookstore.add_item(Item {
                    isbn,
                    title,
                    author,
                    keywords,
                    quantity,
                    item_type,
                });

                println!("Item added successfully.");
            }
            "2" => {
                let isbn = input("Enter ISBN of the item to remove:");
                if bookstore.remove_item(&isbn).is_some() {
                    println!("Item removed successfully.");
                } else {
                    println!("Item not found.");
                }
            }
            "3" => {
                let isbn = input("Enter ISBN of the item to request:");
                if bookstore.request_item(&isbn).is_some() {
                    println!("Item requested successfully.");
                }
            }
            "4" => {
                let isbn = input("Enter ISBN of the item to return:");
                bookstore.return_item(&isbn);
                println!("Item returned successfully.");
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
