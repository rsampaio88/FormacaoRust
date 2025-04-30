/*
 * ================================================================================
 * Author:     Rita Ferreira
 * File:       warehouse.rs
 * Purpose:    - Implement the interactive terminal menu.
 *             - Defines the Warehouse struct to store the layout
 *             (rows, shelves, zones) and allocated items.
 *             - Includes functions for adding, removing, and searching items.
 *             - Includes item searches by ID or name and checks for expiry dates
 * ================================================================================
 */

/*
Notes:

    warehouse
    |
    ----- rows
          |
          -----shelves
              |
              -----zones
                   |
                   -----item


        (1): if last_index = 4  & all_zones.len() = 6 :   4 -> 5 -> 0 -> 1 -> 2 -> 3
*/

use crate::item::Item;

pub enum AllocationStrategy {
    Closest, // puts items closer to the entrance
    Robin,   // Round-robin allocation, cycles through zones
}

pub struct Warehouse {
    rows: Vec<Row>,
    allocation_strategy: AllocationStrategy,
    last_used_index: usize, // for round-robin
}

impl Warehouse {
    pub fn new(strategy: AllocationStrategy) -> Warehouse {
        Warehouse {
            rows: Vec::new(),
            allocation_strategy: strategy,
            last_used_index: 0, //starting
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn find_zone_for_item(&mut self, item: &Item) -> Option<(usize, usize, usize)> {
        match self.allocation_strategy {
            AllocationStrategy::Closest => self.find_closest(item),
            AllocationStrategy::Robin => self.find_robin(item),
        }
    }

    // find first empty zone
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

    // allocate cyclically
    fn find_robin(&mut self, _item: &Item) -> Option<(usize, usize, usize)> {
        let mut all_zones: Vec<(usize, usize, usize)> = vec![]; // vector with all zones indexs

        for (r_idx, row) in self.rows.iter().enumerate() {
            for (s_idx, shelf) in row.shelves.iter().enumerate() {
                for (z_idx, zone) in shelf.zones.iter().enumerate() {
                    all_zones.push((r_idx, s_idx, z_idx));
                }
            }
        }

        let total = all_zones.len();
        //println!("Total zones: {}, Starting from index: {}", total, self.last_used_index);

        for i in 0..total {
            let index = (self.last_used_index + i) % total;
            let (r, s, z) = all_zones[index];

            //println!("zone at Row {}, Shelf {}, Zone {}", r, s, z);

            if self.rows[r].shelves[s].zones[z].item.is_none() {
                //println!("found empty zone at Row {}, Shelf {}, Zone {}", r, s, z);

                self.last_used_index = (index + 1) % total; // Update pointer

                //println!("next start index will be: {}", self.last_used_index);

                return Some((r, s, z));
            }
        }
        // notes (1)
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
        Row {
            shelves: Vec::new(),
        }
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
