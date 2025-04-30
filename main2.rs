/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Brief description of the program
 * =======================================================================
 */

 use crate::item::Item;
use crate::warehouse::{Warehouse, AllocationStrategy};
use crate::filters::{Filter, ExpirationFilter, MaxFileiraFilter};
use std::io::{self, Write};

mod item;
mod warehouse;
mod filters;

fn main() {
    let mut warehouse = Warehouse::new(AllocationStrategy::Robin);

    // Initialize filters
    let mut filters: Vec<Box<dyn Filter>> = Vec::new();
    filters.push(Box::new(MaxFileiraFilter::new(3))); // Max row for fragile items
    filters.push(Box::new(ExpirationFilter::new())); // Expiration filter

    loop {
        // Print menu options
        println!("\nWarehouse Manager Menu:");
        println!("1. Add Item");
        println!("2. Search Item by ID");
        println!("3. Search Item by Name");
        println!("4. Remove Item by ID");
        println!("5. Search Item Zone");
        println!("6. Check Expiring Items");
        println!("7. Exit");

        print!("Please choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                // Add Item
                let item = create_item();
                if filters.iter().all(|f| f.apply(&warehouse, &item)) {
                    if let Some(zone) = warehouse.find_zone_for_item(&item) {
                        warehouse.add_item_to_zone(item, 0, 0, 0); // Assign to zone (row 0, shelf 0, zone 0)
                        println!("Item added to warehouse.");
                    }
                }
            },
            "2" => {
                // Search Item by ID
                print!("Enter the item ID: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let item_id: u32 = id_input.trim().parse().unwrap();
                
                match warehouse.search_item_by_id(item_id) {
                    Some(item) => println!("Item found: {:?}", item),
                    None => println!("Item with ID {} not found", item_id),
                }
            },
            "3" => {
                // Search Item by Name
                print!("Enter the item name: ");
                io::stdout().flush().unwrap();
                let mut name_input = String::new();
                io::stdin().read_line(&mut name_input).unwrap();
                let item_name = name_input.trim();

                let found_items = warehouse.search_item_by_name(item_name);
                if !found_items.is_empty() {
                    println!("Items found with name '{}': {:?}", item_name, found_items);
                } else {
                    println!("No items found with name '{}'", item_name);
                }
            },
            "4" => {
                // Remove Item by ID
                print!("Enter the item ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let item_id: u32 = id_input.trim().parse().unwrap();

                if warehouse.remove_item_by_id(item_id) {
                    println!("Item with ID {} removed from warehouse", item_id);
                } else {
                    println!("Item with ID {} not found for removal", item_id);
                }
            },
            "5" => {
                // Search Item Zone
                print!("Enter the item ID to search for its zone: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let item_id: u32 = id_input.trim().parse().unwrap();

                match warehouse.search_item_zone(item_id) {
                    Some((row, shelf, zone)) => println!(
                        "Item found in row {}, shelf {}, zone {}",
                        row, shelf, zone
                    ),
                    None => println!("Item with ID {} not found in any zone", item_id),
                }
            },
            "6" => {
                // Check Expiring Items
                let today = "2025-04-27"; // Assume today is 2025-04-27
                let expiring_items = warehouse.check_expiring_items(today);
                println!("Items expiring within 3 days: {:?}", expiring_items);
            },
            "7" => {
                // Exit
                println!("Exiting Warehouse Manager...");
                break;
            },
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

// Helper function to simulate item creation (you can replace this with real data input)
fn create_item() -> Item {
    print!("Enter item ID: ");
    io::stdout().flush().unwrap();
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).unwrap();
    let item_id: u32 = id_input.trim().parse().unwrap();

    print!("Enter item name: ");
    io::stdout().flush().unwrap();
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let item_name = name_input.trim().to_string();

    print!("Enter item quantity: ");
    io::stdout().flush().unwrap();
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).unwrap();
    let item_quantity: u32 = quantity_input.trim().parse().unwrap();

    print!("Enter item quality (Fragile, Normal, Oversized): ");
    io::stdout().flush().unwrap();
    let mut quality_input = String::new();
    io::stdin().read_line(&mut quality_input).unwrap();
    let item_quality = quality_input.trim().to_string();

    let item_expiration_date = if item_quality == "Fragile" {
        print!("Enter expiration date (YYYY-MM-DD): ");
        io::stdout().flush().unwrap();
        let mut expiration_input = String::new();
        io::stdin().read_line(&mut expiration_input).unwrap();
        expiration_input.trim().to_string()
    } else {
        "".to_string()
    };

    Item::new(item_id, item_name, item_quantity, item_quality, item_expiration_date)
}
