
struct warehouse {
    rows: Vec<row>,
}

impl Warehouse {

    fn 
}


struct row {
    shelves: Vec<shelf>,
}

struct shelf {
    zones: Vec<zone>,
}

struct zone {
    itens: Option<item>
}

struct item {
    id: u32,
    name: String,
    quantity: u32,
    quanlity: quality,
    timestamp: u64,
}

enum quality {
    fragile {expiration_date: u64, max_row: u32},
    oversized {required_zones: u32},
    normal,
}





