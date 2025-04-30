/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Entry point for the grocery store program.
 * =======================================================================
 */

mod item;
mod store;

use item::Item;
use item::Product;
use std::io::{self, Write};
use store::{GroceryStore, Location, StoreError};

fn main() {
    let mut store = GroceryStore::<Product>::new();

    loop {
        println!("\n--- Menu ---");
        println!("1: Add product");
        println!("2: Remove product");
        println!("3: Move product");
        println!("4: Change product name");
        println!("5: Change product price");
        println!("6: Restock product");
        println!("7: Show inventory");
        println!("8: Find product location");
        println!("9: Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "1" => {
                let (loc, product) = get();
                match store.add_product(loc, product) {
                    Ok(_) => println!("SUCESS!!!"),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "2" => {
                let id = input("Enter product ID to remove:");
                match store.remove_product(&id) {
                    Ok(_) => println!("SUCESS!!!"),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "3" => {
                let id = input("Enter product ID to move:");
                let new_loc = get_location();
                match store.move_product(&id, new_loc) {
                    Ok(_) => println!("SUCESS!!!"),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "4" => {
                let id = input("Enter product ID to rename:");
                let name = input("Enter new name:");
                match store.update_name(&id, name) {
                    Ok(_) => println!("SUCESS!!!"),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "5" => {
                let id = input("Enter product ID to update price:");
                let price_str = input("Enter new price:");
                if let Ok(price) = price_str.parse::<f64>() {
                    match store.update_price(&id, price) {
                        Ok(_) => println!("SUCESS!!"),
                        Err(e) => println!("Error: {:?}", e),
                    }
                } else {
                    println!("Sorry, invalid price.");
                }
            }
            "6" => {
                let id = input("Enter product ID:");
                let amount_str = input("Enter amount to add/remove:");
                if let Ok(amount) = amount_str.parse::<i32>() {
                    match store.restock(&id, amount) {
                        Ok(_) => println!("SUCESS!!!"),
                        Err(e) => println!("Error: {:?}", e),
                    }
                } else {
                    println!("Sorry, invalid amount.");
                }
            }
            "7" => store.print_inventory(),
            "8" => {
                let id = input("Enter product ID to search:");
                match store.find_product(&id) {
                    Some(loc) => println!(
                        "Product '{}' is at Row: {}, Shelf: {}, Zone: {}",
                        id, loc.row, loc.shelf, loc.zone
                    ),
                    None => println!("Sorry, product not found."),
                }
            }
            "9" => break,
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
