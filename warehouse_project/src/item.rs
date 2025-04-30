/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       item.rs
 * Purpose:    Represents items with quantity, quality types, and 
 *            expiration logic for fragile goods.
 * =======================================================================
 */


use chrono::{NaiveDate, Utc};

#[derive(Debug)]
pub enum Quality {
    Fragile { expiry_date: String, max_shelf: u32 },
    Oversized { zones_needed: u32 },
    Normal,
}

pub struct Item {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
    pub quality: Quality,
    pub timestamp: String,
}

impl Item {
    pub fn new(id: u32, name: String, quantity: u32, quality: Quality, timestamp: String) -> Self {
        Item {
            id,
            name,
            quantity,
            quality,
            timestamp,
        }
    }

    pub fn details(&self) -> String {
        format!(
            "ID: {}, Name: {}, Quantity: {}, Quality: {:?}, Timestamp: {}",
            self.id, self.name, self.quantity, self.quality, self.timestamp
        )
    }

    pub fn is_expired(&self) -> bool {
        if let Quality::Fragile { expiry_date, .. } = &self.quality {
            if let Ok(date) = NaiveDate::parse_from_str(&expiry_date, "%Y-%m-%d") {
                let today = Utc::now().naive_utc().date();
                return date < today;
            }
        }
        false
    }
}
