/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Store implementation with inventory and error definitions.
 * =======================================================================
 */

use crate::item::Item;
use std::collections::HashMap;

#[derive(Debug)]
pub enum StoreError {
    ProductNotFound,
    LocationNotFound,
    NotEnoughStock,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Location {
    pub row: String,
    pub shelf: String,
    pub zone: String,
}

pub struct GroceryStore<T: Item> {
    inventory: HashMap<Location, Vec<T>>,
    product_locations: HashMap<String, Location>,
}

impl<T: Item> GroceryStore<T> {
    pub fn new() -> Self {
        GroceryStore {
            inventory: HashMap::new(),
            product_locations: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, location: Location, product: T) -> Result<(), StoreError> {
        self.product_locations
            .insert(product.id().to_string(), location.clone());
        self.inventory.entry(location).or_default().push(product);
        Ok(())
    }

    pub fn remove_product(&mut self, id: &str) -> Result<(), StoreError> {
        let location = self
            .product_locations
            .remove(id)
            .ok_or(StoreError::ProductNotFound)?;
        let products = self
            .inventory
            .get_mut(&location)
            .ok_or(StoreError::LocationNotFound)?;

        if let Some(pos) = products.iter().position(|p| p.id() == id) {
            products.remove(pos);
            Ok(())
        } else {
            Err(StoreError::ProductNotFound)
        }
    }

    pub fn move_product(&mut self, id: &str, new_location: Location) -> Result<(), StoreError> {
        let current_location = self
            .product_locations
            .get(id)
            .cloned()
            .ok_or(StoreError::ProductNotFound)?;
        let products = self
            .inventory
            .get_mut(&current_location)
            .ok_or(StoreError::LocationNotFound)?;

        if let Some(pos) = products.iter().position(|p| p.id() == id) {
            let product = products.remove(pos);
            self.add_product(new_location.clone(), product)?;
            self.product_locations.insert(id.to_string(), new_location);
            Ok(())
        } else {
            Err(StoreError::ProductNotFound)
        }
    }

    pub fn update_price(&mut self, id: &str, new_price: f64) -> Result<(), StoreError> {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id() == id) {
                p.set_price(new_price);
                return Ok(());
            }
        }
        Err(StoreError::ProductNotFound)
    }

    pub fn update_name(&mut self, id: &str, new_name: String) -> Result<(), StoreError> {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id() == id) {
                p.set_name(new_name);
                return Ok(());
            }
        }
        Err(StoreError::ProductNotFound)
    }

    pub fn restock(&mut self, id: &str, amount: i32) -> Result<(), StoreError> {
        for products in self.inventory.values_mut() {
            if let Some(p) = products.iter_mut().find(|p| p.id() == id) {
                return p.restock(amount);
            }
        }
        Err(StoreError::ProductNotFound)
    }

    pub fn find_product(&self, id: &str) -> Option<&Location> {
        self.product_locations.get(id)
    }

    pub fn print_inventory(&self) {
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
                    p.name(),
                    p.id(),
                    p.quantity(),
                    p.expiration_date(),
                    p.price()
                );
            }
        }
    }
}
