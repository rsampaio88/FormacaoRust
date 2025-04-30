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

    fn find_items_by_keyword(&self, keyword: &str) {
        let found_items = false;
        for item in self.items.values() {
            if item.keywords.contains(&keyword.to_string()) {
                println!(
                    "Found item: '{}' by {} (ISBN: {})",
                    item.title, item.author, item.isbn
                )
            }
        }

        if !found_items {
            println!("Sorry, no items found with the keyword '{}'", keyword);
        }
    }
}

fn main() {
    let mut bookstore = Bookstore::new();

    bookstore.add_item(Item {
        isbn: "978-0999999999".to_string(),
        title: " History of the Roman Empire".to_string(),
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
        println!("\n--- Bookstore Menu ---");
        println!("1: Add an item");
        println!("2: Remove an item");
        println!("3: Request an item");
        println!("4: Return an item");
        println!("5: Search for items by keyword");
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
                let mut item_type = String::new();

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
                println!("Enter item type (1: Book, 2: AudioBook, 3: Statue, 4:Painting): ");
                io::stdin()
                    .read_line(&mut item_type)
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
                let item_type = match item_type.trim() {
                    "1" => ItemType::Book,
                    "2" => ItemType::Audiobook,
                    "3" => ItemType::Statue,
                    "4" => ItemType::Painting,
                    _ => {
                        println!("Sorry don't recognise that type. By default it will be book");
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
                let mut isbn = String::new();
                println!("Enter ISBN of the item to remove: ");
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read line");

                let isbn = isbn.trim();
                if bookstore.remove_item(isbn).is_some() {
                    println!("Item removed successfully.");
                } else {
                    println!("Item not found.");
                }
            }
            "3" => {
                let mut isbn = String::new();
                println!("Enter ISBN of the item to request: ");
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read line");

                let isbn = isbn.trim();
                if bookstore.request_item(isbn).is_some() {
                    println!("Item requested successfully.");
                }
            }
            "4" => {
                let mut isbn = String::new();
                println!("Enter ISBN of the item to return: ");
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read line");

                let isbn = isbn.trim();
                bookstore.return_item(isbn);
                println!("Item returned successfully.");
            }
            "5" => {
                let mut keyword = String::new();
                println!("Enter a keyword to search for: ");
                io::stdin()
                    .read_line(&mut keyword)
                    .expect("Failed to read line");

                let keyword = keyword.trim().to_string();
                bookstore.find_items_by_keyword(&keyword);
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
