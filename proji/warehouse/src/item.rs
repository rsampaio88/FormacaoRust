/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       item.rs
 * Purpose:    - Defines the Item struct with fields like id, name, 
 *              quantity, quality, and timestamp.
 * 
 *             - Defines the Quality enum with categories: Fragile, 
 *              Oversized, and Normal, each with specific properties.
 * 
 *             - Includes methods for creating and managing items 
 *              and accessing or modifying their data.
 * =======================================================================
 */


/*
Notes:

    format! : creates string.
 */

pub struct Item {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
    pub quality: Quality,
    pub timestamp: String,
}


pub enum Quality {
    Fragile { expiry_date: String, max_shelf: u32 },
    Oversized { zones_needed: u32 },
    Normal,
}

impl Item {
    
    pub fn new(id: u32, name: String, quantity: u32, quality: Quality, timestamp: String) -> Item {
        Item {
            id,
            name,
            quantity,
            quality,
            timestamp,
        }
    }
    
    // get item's information
    pub fn details(&self) -> String {
        format!("ID: {}, Name: {}, Quantity: {}, Quality: {:?}, Timestamp: {}",
            self.id, self.name, self.quantity, self.quality, self.timestamp
        )
    }
}

