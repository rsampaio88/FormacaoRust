/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Main program logic for managing a warehouse inventory
 *            system with filtering and placement strategies.
 * =======================================================================
 */

mod filter;
mod item;
mod utils;
mod warehouse;

use crate::item::{Item, Quality};
use crate::utils::{parse_input, read_input};
use crate::warehouse::{AllocationStrategy, Row, Shelf, Warehouse, Zone};
use chrono::{NaiveDate, Utc};

fn main() {
    println!("Welcome Mr.Inventory Manager");

    let mut warehouse = Warehouse::new(AllocationStrategy::Closest);

    for _ in 0..2 {
        let mut row = Row::new();
        for _ in 0..2 {
            let mut shelf = Shelf::new();
            for _ in 0..3 {
                shelf.add_zone(Zone::new());
            }
            row.add_shelf(shelf);
        }
        warehouse.add_row(row);
    }

    loop {
        println!("\n===== INVENTORY MANAGER =====");
        println!("1. Add new item");
        println!("2. Search item by ID");
        println!("3. Search item by name");
        println!("4. Find all item locations");
        println!("5. Remove item from zone");
        println!("6. Show all items");
        println!("7. Show items close to expiry");
        println!("8. Exit");
        println!("==================================");

        let option = read_input("Choose an option: ");

        match option.as_str() {
            "1" => add_item(&mut warehouse),
            "2" => search_by_id(&warehouse),
            "3" => search_by_name(&warehouse),
            "4" => find_locations(&warehouse),
            "5" => remove_item(&mut warehouse),
            "6" => show_all(&warehouse),
            "7" => show_near_expiry(&warehouse),
            "8" => {
                println!("Leaving...\nSee you soon");
                break;
            }
            _ => println!("Sorry, Invalid choice. Please try again."),
        }
    }
}

fn add_item(warehouse: &mut Warehouse) {
    use crate::filter::{ExpirationFilter, Filter, MaxRow};

    let id = parse_input("Enter item ID: ") as u32;
    let name = read_input("Enter item name: ");
    let quantity = parse_input("Enter quantity: ") as u32;

    println!("Choose item quality:");
    println!("1. Normal");
    println!("2. Fragile");
    println!("3. Oversized");
    let quality_choice = read_input("Your choice: ");

    let quality = match quality_choice.as_str() {
        "1" => Quality::Normal,
        "2" => {
            let expiry = read_input("Enter expiry date (YYYY-MM-DD): ");
            let max_shelf = parse_input("Enter max row number for fragile item: ") as u32;
            Quality::Fragile {
                expiry_date: expiry,
                max_shelf,
            }
        }
        "3" => {
            let zones = parse_input("Enter number of zones needed: ") as u32;
            Quality::Oversized {
                zones_needed: zones,
            }
        }
        _ => {
            println!("Invalid quality selected, defaulting to Normal.");
            Quality::Normal
        }
    };

    let timestamp = chrono::Utc::now().timestamp().to_string();
    let item = Item::new(id, name, quantity, quality, timestamp);

    let filters: Vec<Box<dyn Filter>> =
        vec![Box::new(MaxRow::new(2)), Box::new(ExpirationFilter::new())];

    for filter in &filters {
        if !filter.apply(&warehouse, &item) {
            println!("Item rejected by a warehouse filter.");
            return;
        }
    }

    if let Some((r, s, z)) = warehouse.find_zone(&item) {
        warehouse.add_zone(item, r, s, z);
        println!("Item stored at Row {}, Shelf {}, Zone {}", r, s, z);
    } else {
        println!("No space available for the item.");
    }
}

fn search_by_id(warehouse: &Warehouse) {
    let id = parse_input("Enter ID to search: ") as u32;
    let mut count = 0;

    for (r_idx, row) in warehouse.rows.iter().enumerate() {
        for (s_idx, shelf) in row.shelves.iter().enumerate() {
            for (z_idx, zone) in shelf.zones.iter().enumerate() {
                if let Some(item) = &zone.item {
                    if item.id == id {
                        count += 1;
                        println!("Found at Row {}, Shelf {}, Zone {}", r_idx, s_idx, z_idx);
                    }
                }
            }
        }
    }

    println!("Total items with ID {}: {}", id, count);
}

fn search_by_name(warehouse: &Warehouse) {
    let name = read_input("Enter name to search: ");
    let mut count = 0;

    for row in &warehouse.rows {
        for shelf in &row.shelves {
            for zone in &shelf.zones {
                if let Some(item) = &zone.item {
                    if item.name == name {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Total items named '{}': {}", name, count);
}

fn find_locations(warehouse: &Warehouse) {
    let id = parse_input("Enter item ID: ") as u32;

    for (r_idx, row) in warehouse.rows.iter().enumerate() {
        for (s_idx, shelf) in row.shelves.iter().enumerate() {
            for (z_idx, zone) in shelf.zones.iter().enumerate() {
                if let Some(item) = &zone.item {
                    if item.id == id {
                        println!("Row {}, Shelf {}, Zone {}", r_idx, s_idx, z_idx);
                    }
                }
            }
        }
    }
}

fn remove_item(warehouse: &mut Warehouse) {
    let row = parse_input("Enter row: ");
    let shelf = parse_input("Enter shelf: ");
    let zone = parse_input("Enter zone: ");

    if row < warehouse.rows.len()
        && shelf < warehouse.rows[row].shelves.len()
        && zone < warehouse.rows[row].shelves[shelf].zones.len()
    {
        warehouse.rows[row].shelves[shelf].zones[zone].item = None;
        println!(
            "Item removed from Row {}, Shelf {}, Zone {}",
            row, shelf, zone
        );
    } else {
        println!("Sorry, invalid location.");
    }
}

fn show_all(warehouse: &Warehouse) {
    let mut items: Vec<&Item> = vec![];

    for row in &warehouse.rows {
        for shelf in &row.shelves {
            for zone in &shelf.zones {
                if let Some(item) = &zone.item {
                    items.push(item);
                }
            }
        }
    }

    items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    for item in items {
        println!("{}", item.details());
    }
}

fn show_near_expiry(warehouse: &Warehouse) {
    let mut count = 0;
    let today = Utc::now().naive_utc().date();

    for row in &warehouse.rows {
        for shelf in &row.shelves {
            for zone in &shelf.zones {
                if let Some(item) = &zone.item {
                    if let Quality::Fragile { expiry_date, .. } = &item.quality {
                        if let Ok(expiry) = NaiveDate::parse_from_str(expiry_date, "%Y-%m-%d") {
                            let days_left = (expiry - today).num_days();
                            if days_left <= 3 {
                                println!(
                                    "Item '{}' expires in {} days (ID: {})",
                                    item.name, days_left, item.id
                                );
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Total items that expire: {}", count);
}
