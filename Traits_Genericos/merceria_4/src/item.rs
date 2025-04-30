/*
* =======================================================================
* Author:     Rita Ferreira
* File:       item.rs
* Purpose:    Contains the Item trait and its implementation for
              the Product type.
* =======================================================================
*/

use crate::store::StoreError;

pub trait Item: Clone {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn expiration_date(&self) -> &str;
    fn price(&self) -> f64;
    fn quantity(&self) -> u32;

    fn set_name(&mut self, new_name: String);
    fn set_price(&mut self, new_price: f64);
    fn restock(&mut self, amount: i32) -> Result<(), StoreError>;
}

#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub expiration_date: String,
    pub price: f64,
    pub quantity: u32,
}

impl Item for Product {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn expiration_date(&self) -> &str {
        &self.expiration_date
    }

    fn price(&self) -> f64 {
        self.price
    }

    fn quantity(&self) -> u32 {
        self.quantity
    }

    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn set_price(&mut self, new_price: f64) {
        self.price = new_price;
    }

    fn restock(&mut self, amount: i32) -> Result<(), StoreError> {
        let new_quantity = self.quantity as i32 + amount;
        if new_quantity < 0 {
            return Err(StoreError::NotEnoughStock);
        }
        self.quantity = new_quantity as u32;
        Ok(())
    }
}
