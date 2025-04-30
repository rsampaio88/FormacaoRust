/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Implementation of a cgrocery store
 *              -
 * =======================================================================
 */

use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Product {
    id: String,
    name: String,
    expiration_date: String,
    price: f64,
    quantity: u32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Location {
    row: String,
    shelf: String,
    zone: String,
}

struct GroceryStore {
    inventory: HashMap<Location, Vec<Product>>,
}

impl GroceryStore {
    fn new() -> Self {
        GroceryStore {
            inventory: HashMap::new(),
        }
    }

    fn add_product(&mut self, location: Location, product: Product) {
        self.inventory.entry(location).or_default().push(product);
    }

    fn remove_product(&mut self, id: &str) -> bool {
        for products in self.inventory.values_mut() {
            if let Some(pos) = products.iter().position(|p| p.id == id) {
                products.remove(pos);
                return true;
            }
        }
        false
    }

    fn move_product(&mut self, id: &str, new_location: Location) -> bool {
        for products in self.inventory.values_mut() {
            if let Some(pos) = products.iter().position(|p| p.id == id) {
                let product = products.remove(pos);
                self.add_product(new_location, product);
                return true;
            }
        }
        false
    }

    fn update_price(&mut self, id: &str, new_price: f64) -> bool {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id == id) {
                p.price = new_price;
                return true;
            }
        }
        false
    }

    fn update_name(&mut self, id: &str, new_name: String) -> bool {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id == id) {
                p.name = new_name;
                return true;
            }
        }
        false
    }

    fn restock(&mut self, id: &str, amount: i32) -> bool {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id == id) {
                let new_quantity = p.quantity as i32 + amount;
                if new_quantity < 0 {
                    println!("Sorry, not enough stock to remove.");
                    return false;
                }
                p.quantity = new_quantity as u32;
                return true;
            }
        }
        false
    }

    fn print_inventory(&self) {
        if self.inventory.is_empty() {
            println!("Sorry, inventory is empty.");
            return;
        }

        for (loc, products) in &self.inventory {
            if products.is_empty() {
                println!(
                    "\nLocation - Row: {}, Shelf: {}, Zone: {} is empty.",
                    loc.row, loc.shelf, loc.zone
                );
                continue;
            }

            println!(
                "\nLocation - Row: {}, Shelf: {}, Zone: {}",
                loc.row, loc.shelf, loc.zone
            );
            for p in products {
                println!(
                    "- {} (Id: {}), Quantity: {}, Expiration Date: {}, Price: {:.2}",
                    p.name, p.id, p.quantity, p.expiration_date, p.price
                );
            }
        }
    }
}

fn main() {
    let mut store = GroceryStore::new();

    loop {
        println!("\n--- Menu ---");
        println!("1: Add product");
        println!("2: Remove product");
        println!("3: Move product");
        println!("4: Change product name");
        println!("5: Change product price");
        println!("6: Restock product");
        println!("7: Show inventory");
        println!("8: Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "1" => {
                let (loc, product) = get();
                store.add_product(loc, product);
                println!("SUCESS!!!");
            }
            "2" => {
                let id = input("Enter product ID to remove:");
                if store.remove_product(&id) {
                    println!("SUCESS!!!");
                } else {
                    println!("Sorry, product not found.");
                }
            }
            "3" => {
                let id = input("Enter product ID to move:");
                let new_loc = get_location();
                if store.move_product(&id, new_loc) {
                    println!("SUCESS!!!");
                } else {
                    println!("Sorry, product not found.");
                }
            }
            "4" => {
                let id = input("Enter product ID to rename:");
                let name = input("Enter new name:");
                if store.update_name(&id, name) {
                    println!("SUCESS!!!");
                } else {
                    println!("Sorry, product not found.");
                }
            }
            "5" => {
                let id = input("Enter product ID to update price:");
                let price_str = input("Enter new price:");
                if let Ok(price) = price_str.parse::<f64>() {
                    if store.update_price(&id, price) {
                        println!("SUCESS!!!");
                    } else {
                        println!("Sorry, product not found.");
                    }
                } else {
                    println!("Sory, invalid price.");
                }
            }
            "6" => {
                let id = input("Enter product ID:");
                let amount_str = input("Enter amount to add or remove:");
                if let Ok(amount) = amount_str.parse::<i32>() {
                    if store.restock(&id, amount) {
                        println!("SUCESS!!.");
                    } else {
                        println!("Sorry, failed to restock.");
                    }
                } else {
                    println!("Sorry, invalid amount.");
                }
            }
            "7" => store.print_inventory(),
            "8" => break,
            _ => println!("Sorry, invalid option."),
        }
    }
}

fn input(text: &str) -> String {
    print!("{} ", text);
    io::stdout().flush().unwrap();
    let mut val = String::new();
    io::stdin().read_line(&mut val).unwrap();
    val.trim().to_string()
}

fn get() -> (Location, Product) {
    let location = get_location();
    let id = input("Product ID:");
    let name = input("Name:");
    let exp = input("Expiration date:");
    let price = input("Price:");
    let quantity = input("Quantity:");

    let price: f64 = price.trim().parse().unwrap_or(0.0);
    let quantity: u32 = quantity.trim().parse().unwrap_or(0);

    let product = Product {
        id,
        name,
        expiration_date: exp,
        price,
        quantity,
    };

    (location, product)
}

fn get_location() -> Location {
    let row = input("Row:");
    let shelf = input("Shelf:");
    let zone = input("Zone:");
    Location { row, shelf, zone }
}
