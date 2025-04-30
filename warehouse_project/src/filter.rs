use crate::item::Item;
use crate::warehouse::Warehouse;

pub trait Filter {
    fn apply(&self, warehouse: &Warehouse, item: &Item) -> bool;
}

pub struct MaxRow {
    pub max_row: usize,
}

impl MaxRow {
    pub fn new(max_row: usize) -> Self {
        MaxRow { max_row }
    }
}

impl Filter for MaxRow {
    fn apply(&self, _warehouse: &Warehouse, item: &Item) -> bool {
        match &item.quality {
            crate::item::Quality::Fragile { max_shelf, .. } => *max_shelf <= self.max_row as u32,
            _ => true,
        }
    }
}

pub struct ExpirationFilter;

impl ExpirationFilter {
    pub fn new() -> Self {
        ExpirationFilter
    }
}

impl Filter for ExpirationFilter {
    fn apply(&self, _warehouse: &Warehouse, item: &Item) -> bool {
        match &item.quality {
            crate::item::Quality::Fragile { .. } => !item.is_expired(),
            _ => true, // normal  oversized not filter
        }
    }
}
