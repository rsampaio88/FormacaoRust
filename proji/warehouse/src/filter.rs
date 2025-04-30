/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       filter.rs
 * Purpose:    - Defines the Filter trait, allowing the creation of 
 *              various filters for items.
 *             - Function that checks if an item can be accepted based 
 *              on the warehouse state.
 * =======================================================================
 */

/*
Notes:

    - assuming that if the expiration date of an item is less than current 
    date => will not be added.

    -

*/
 
use crate::item::Item;
use crate::warehouse::Warehouse;

//filter trait
pub trait Filter {
    fn apply(&self, warehouse: &Warehouse, item: &Item) -> bool;
}

//rejects fragile items in rows greater than max.
pub struct MaxRow {
    max_row: usize, 
}

impl MaxRow {
    pub fn new(max_fileira: usize) -> Self {
        MaxRow { max_row }
    }
}

impl Filter for MaxRow {
    fn apply(&self, warehouse: &Warehouse, item: &Item) -> bool {
        if let crate::item::Quality::Fragile = item.quality {
            // Check the fragile item is allowed row range
            for (row_idx, row) in warehouse.rows.iter().enumerate() {
                if row_idx > self.max_fileira {
                    return false; 
                }
            }
        }
        true
    }
}

pub struct ExpirationFilter;

impl ExpirationFilter {
    pub fn new() -> ExpirationFilter {
        ExpirationFilter
    }
}

impl Filter for ExpirationFilter {
    fn apply(&self, _warehouse: &Warehouse, item: &Item) -> bool {
        // Check expiration
        item.is_expired()
    }
}
