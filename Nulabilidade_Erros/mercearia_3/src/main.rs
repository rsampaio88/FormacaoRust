


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

#[derive(Debug)]
enum StoreError {
    ProductNotFound,
    LocationNotFound,
    NotEnoughStock,
}

struct GroceryStore {
    inventory: HashMap<Location, Vec<Product>>,
    product_locations: HashMap<String, Location>,
}

impl GroceryStore {
    fn new() -> Self {
        GroceryStore {
            inventory: HashMap::new(),
            product_locations: HashMap::new(),
        }
    }

    fn add_product(&mut self, location: Location, product: Product) -> Result<(), StoreError> {
        self.product_locations.insert(product.id.clone(), location.clone());
        self.inventory.entry(location).or_default().push(product);
        Ok(())
    }

    fn remove_product(&mut self, id: &str) -> Result<(), StoreError> {
        let location = self.product_locations.remove(id).ok_or(StoreError::ProductNotFound)?;

        let products = self
            .inventory
            .get_mut(&location)
            .ok_or(StoreError::LocationNotFound)?;

        if let Some(pos) = products.iter().position(|p| p.id == id) {
            products.remove(pos);
            Ok(())
        } else {
            Err(StoreError::ProductNotFound)
        }
    }

    fn move_product(&mut self, id: &str, new_location: Location) -> Result<(), StoreError> {
        let current_location = self
            .product_locations
            .get(id)
            .cloned()
            .ok_or(StoreError::ProductNotFound)?;

        let products = self
            .inventory
            .get_mut(&current_location)
            .ok_or(StoreError::LocationNotFound)?;

        if let Some(pos) = products.iter().position(|p| p.id == id) {
            let product = products.remove(pos);
            self.add_product(new_location.clone(), product)?;
            self.product_locations.insert(id.to_string(), new_location);
            Ok(())
        } else {
            Err(StoreError::ProductNotFound)
        }
    }

    fn update_price(&mut self, id: &str, new_price: f64) -> Result<(), StoreError> {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id == id) {
                p.price = new_price;
                return Ok(());
            }
        }
        Err(StoreError::ProductNotFound)
    }

    fn update_name(&mut self, id: &str, new_name: String) -> Result<(), StoreError> {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id == id) {
                p.name = new_name;
                return Ok(());
            }
        }
        Err(StoreError::ProductNotFound)
    }

    fn restock(&mut self, id: &str, amount: i32) -> Result<(), StoreError> {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id == id) {
                let new_quantity = p.quantity as i32 + amount;
                if new_quantity < 0 {
                    return Err(StoreError::NotEnoughStock);
                }
                p.quantity = new_quantity as u32;
                return Ok(());
            }
        }
        Err(StoreError::ProductNotFound)
    }

    fn find_product(&self, id: &str) -> Option<&Location> {
        self.product_locations.get(id)
    }

    fn print_inventory(&self) {
        if self.inventory.is_empty() {
            println!("It's empty.");
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
                    Ok(_) => println!("Product added."),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "2" => {
                let id = input("Enter product ID to remove:");
                match store.remove_product(&id) {
                    Ok(_) => println!("Product removed."),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "3" => {
                let id = input("Enter product ID to move:");
                let new_loc = get_location();
                match store.move_product(&id, new_loc) {
                    Ok(_) => println!("Product moved."),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "4" => {
                let id = input("Enter product ID to rename:");
                let name = input("Enter new name:");
                match store.update_name(&id, name) {
                    Ok(_) => println!("Name updated."),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "5" => {
                let id = input("Enter product ID to update price:");
                let price_str = input("Enter new price:");
                if let Ok(price) = price_str.parse::<f64>() {
                    match store.update_price(&id, price) {
                        Ok(_) => println!("Price updated."),
                        Err(e) => println!("Error: {:?}", e),
                    }
                } else {
                    println!("Sorry, invalid price.");
                }
            }
            "6" => {
                let id = input("Enter product ID:");
                let amount_str = input("Enter amount to add or remove:");
                if let Ok(amount) = amount_str.parse::<i32>() {
                    match store.restock(&id, amount) {
                        Ok(_) => println!("Restock successful."),
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
                    None => println!("Product not found."),
                }
            }
            "9" => break,
            _ => println!("Invalid option."),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{} ", prompt);
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

