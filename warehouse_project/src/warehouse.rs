/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       warehouse.rs
 * Purpose:    Brief description of the program
 * =======================================================================
 */


use crate::item::Item;

pub enum AllocationStrategy {
    Closest,
    Robin,
}

pub struct Warehouse {
    pub rows: Vec<Row>,
    allocation_strategy: AllocationStrategy,
    last_used_index: usize,
}

impl Warehouse {
    pub fn new(strategy: AllocationStrategy) -> Warehouse {
        Warehouse {
            rows: Vec::new(),
            allocation_strategy: strategy,
            last_used_index: 0,
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn find_zone(&mut self, item: &Item) -> Option<(usize, usize, usize)> {
        match self.allocation_strategy {
            AllocationStrategy::Closest => self.find_closest(item),
            AllocationStrategy::Robin => self.find_robin(item),
        }
    }

    fn find_closest(&self, _item: &Item) -> Option<(usize, usize, usize)> {
        for (r_idx, row) in self.rows.iter().enumerate() {
            for (s_idx, shelf) in row.shelves.iter().enumerate() {
                for (z_idx, zone) in shelf.zones.iter().enumerate() {
                    if zone.item.is_none() {
                        return Some((r_idx, s_idx, z_idx));
                    }
                }
            }
        }
        None
    }

    fn find_robin(&mut self, _item: &Item) -> Option<(usize, usize, usize)> {
        let mut all_zones: Vec<(usize, usize, usize)> = vec![];

        for (r_idx, row) in self.rows.iter().enumerate() {
            for (s_idx, shelf) in row.shelves.iter().enumerate() {
                for (z_idx, _zone) in shelf.zones.iter().enumerate() {
                    all_zones.push((r_idx, s_idx, z_idx));
                }
            }
        }

        let total = all_zones.len();
        for i in 0..total {
            let index = (self.last_used_index + i) % total;
            let (r, s, z) = all_zones[index];

            if self.rows[r].shelves[s].zones[z].item.is_none() {
                self.last_used_index = (index + 1) % total;
                return Some((r, s, z));
            }
        }
        None
    }

    pub fn add_to_zone(&mut self, item: Item, row: usize, shelf: usize, zone: usize) {
        self.rows[row].shelves[shelf].zones[zone].item = Some(item);
    }
}

pub struct Row {
    pub shelves: Vec<Shelf>,
}

impl Row {
    pub fn new() -> Row {
        Row { shelves: Vec::new() }
    }

    pub fn add_shelf(&mut self, shelf: Shelf) {
        self.shelves.push(shelf);
    }
}

pub struct Shelf {
    pub zones: Vec<Zone>,
}

impl Shelf {
    pub fn new() -> Shelf {
        Shelf { zones: Vec::new() }
    }

    pub fn add_zone(&mut self, zone: Zone) {
        self.zones.push(zone);
    }
}

pub struct Zone {
    pub item: Option<Item>,
}

impl Zone {
    pub fn new() -> Zone {
        Zone { item: None }
    }
}
