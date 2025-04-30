
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// Implementação do gestor de inventário modular

// DEFINIÇÃO DAS ESTRUTURAS BÁSICAS

// Localização de um item no armazém
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    row: u32,     // Fileira
    shelf: u32,   // Prateleira
    zone: u32,    // Zona
}

impl Location {
    fn new(row: u32, shelf: u32, zone: u32) -> Self {
        Location { row, shelf, zone }
    }
    
    // Método para comparar proximidade à base (quanto menor o valor, mais próximo)
    fn proximity_value(&self) -> u32 {
        self.row * 10000 + self.shelf * 100 + self.zone
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fileira: {}, Prateleira: {}, Zona: {}", self.row, self.shelf, self.zone)
    }
}

// Tipos de qualidade de item
enum Quality {
    Fragile { expiry_date: String, max_row: u32 },
    Oversized { zones_needed: u32 },
    Normal,
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Quality::Fragile { expiry_date, max_row } => {
                write!(f, "Frágil (Val: {}, Máx Fileira: {})", expiry_date, max_row)
            }
            Quality::Oversized { zones_needed } => {
                write!(f, "Oversized (Zonas: {})", zones_needed)
            }
            Quality::Normal => {
                write!(f, "Normal")
            }
        }
    }
}

// Estrutura do Item
#[derive(Clone)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
    quality: Quality,
    timestamp: u64,
}

impl Item {
    fn new(id: u32, name: String, quantity: u32, quality: Quality) -> Self {
        // Obtém o timestamp atual em segundos
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Item { id, name, quantity, quality, timestamp }
    }
    
    // Verifica se um item está próximo da data de validade (3 dias ou menos)
    fn is_near_expiry(&self, current_date: &str) -> bool {
        if let Quality::Fragile { expiry_date, .. } = &self.quality {
            // Implementação simplificada: compara strings diretamente
            // Em uma implementação real, seria melhor usar uma biblioteca de datas
            
            // Formato da data: YYYY-MM-DD
            if expiry_date <= current_date {
                return true;
            }
            
            // Verificar se está a 3 dias ou menos
            // Simplificação: assumimos que as datas estão no formato YYYY-MM-DD
            if expiry_date.len() == 10 && current_date.len() == 10 {
                let exp_year: u32 = expiry_date[0..4].parse().unwrap_or(0);
                let exp_month: u32 = expiry_date[5..7].parse().unwrap_or(0);
                let exp_day: u32 = expiry_date[8..10].parse().unwrap_or(0);
                
                let cur_year: u32 = current_date[0..4].parse().unwrap_or(0);
                let cur_month: u32 = current_date[5..7].parse().unwrap_or(0);
                let cur_day: u32 = current_date[8..10].parse().unwrap_or(0);
                
                // Converter para dias (simplificado)
                let exp_days = exp_year * 365 + exp_month * 30 + exp_day;
                let cur_days = cur_year * 365 + cur_month * 30 + cur_day;
                
                return exp_days.saturating_sub(cur_days) <= 3;
            }
        }
        false
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "ID: {}, Nome: {}, Quantidade: {}, Qualidade: {}",
            self.id, self.name, self.quantity, self.quality
        )
    }
}

// ESTRATÉGIAS DE ALOCAÇÃO

// Trait para estratégia de alocação
trait AllocationStrategy {
    fn allocate(&self, warehouse: &Warehouse, item: &Item) -> Option<Location>;
}

// Estratégia 1: Proximidade - escolhe a zona mais próxima da entrada
struct ProximityStrategy;

impl AllocationStrategy for ProximityStrategy {
    fn allocate(&self, warehouse: &Warehouse, item: &Item) -> Option<Location> {
        // Obter todas as localizações disponíveis
        let mut available_locations = warehouse.get_available_locations();
        
        // Verificar se é um item frágil e excluir localizações inválidas
        if let Quality::Fragile { max_row, .. } = item.quality {
            available_locations.retain(|loc| loc.row <= max_row);
        }
        
        // Verificar se é um item oversized
        if let Quality::Oversized { zones_needed } = item.quality {
            if zones_needed > 1 {
                // Filtra para considerar apenas localizações com zonas consecutivas disponíveis
                available_locations.retain(|loc| {
                    warehouse.has_consecutive_free_zones(loc, zones_needed)
                });
            }
        }
        
        // Ordena por proximidade e retorna a mais próxima
        available_locations.sort_by_key(|loc| loc.proximity_value());
        available_locations.first().cloned()
    }
}

// Estratégia 2: Round Robin - distribui os itens de forma circular
struct RoundRobinStrategy {
    last_row: u32,
    last_shelf: u32,
    last_zone: u32,
}

impl RoundRobinStrategy {
    fn new() -> Self {
        RoundRobinStrategy {
            last_row: 0,
            last_shelf: 0,
            last_zone: 0,
        }
    }
}

impl AllocationStrategy for RoundRobinStrategy {
    fn allocate(&self, warehouse: &Warehouse, item: &Item) -> Option<Location> {
        // Obter dimensões máximas do armazém
        let (max_row, max_shelf, max_zone) = warehouse.get_dimensions();
        
        // Começar a busca a partir da última posição usada
        let mut row = self.last_row;
        let mut shelf = self.last_shelf;
        let mut zone = self.last_zone + 1; // Próxima zona
        
        // Ajustar para não exceder os limites
        if zone >= max_zone {
            zone = 0;
            shelf += 1;
        }
        if shelf >= max_shelf {
            shelf = 0;
            row += 1;
        }
        if row >= max_row {
            row = 0; // Voltar ao início
        }
        
        // Verificar uma volta completa
        let initial_row = row;
        let initial_shelf = shelf;
        let initial_zone = zone;
        
        loop {
            let location = Location::new(row, shelf, zone);
            
            // Verificar se a localização está disponível
            if warehouse.is_location_free(&location) {
                // Para itens frágeis, verificar se está dentro da fileira máxima
                if let Quality::Fragile { max_row: max_fragile_row, .. } = item.quality {
                    if row > max_fragile_row {
                        // Continuar para a próxima localização
                        zone += 1;
                        if zone >= max_zone {
                            zone = 0;
                            shelf += 1;
                        }
                        if shelf >= max_shelf {
                            shelf = 0;
                            row += 1;
                        }
                        if row >= max_row {
                            row = 0;
                        }
                        
                        // Verificar se já completamos uma volta
                        if row == initial_row && shelf == initial_shelf && zone == initial_zone {
                            return None; // Nenhuma localização disponível
                        }
                        continue;
                    }
                }
                
                // Para itens oversized, verificar se há zonas contíguas suficientes
                if let Quality::Oversized { zones_needed } = item.quality {
                    if zones_needed > 1 && !warehouse.has_consecutive_free_zones(&location, zones_needed) {
                        // Continuar para a próxima localização
                        zone += 1;
                        if zone >= max_zone {
                            zone = 0;
                            shelf += 1;
                        }
                        if shelf >= max_shelf {
                            shelf = 0;
                            row += 1;
                        }
                        if row >= max_row {
                            row = 0;
                        }
                        
                        // Verificar se já completamos uma volta
                        if row == initial_row && shelf == initial_shelf && zone == initial_zone {
                            return None; // Nenhuma localização disponível
                        }
                        continue;
                    }
                }
                
                // Atualizar a última posição usada
                let mut strategy = self.clone();
                strategy.last_row = row;
                strategy.last_shelf = shelf;
                strategy.last_zone = zone;
                
                return Some(location);
            }
            
            // Avançar para a próxima localização
            zone += 1;
            if zone >= max_zone {
                zone = 0;
                shelf += 1;
            }
            if shelf >= max_shelf {
                shelf = 0;
                row += 1;
            }
            if row >= max_row {
                row = 0;
            }
            
            // Verificar se já completamos uma volta
            if row == initial_row && shelf == initial_shelf && zone == initial_zone {
                return None; // Nenhuma localização disponível
            }
        }
    }
}

impl Clone for RoundRobinStrategy {
    fn clone(&self) -> Self {
        RoundRobinStrategy {
            last_row: self.last_row,
            last_shelf: self.last_shelf,
            last_zone: self.last_zone,
        }
    }
}

// FILTROS DE ACEITAÇÃO

// Trait para filtros
trait ItemFilter {
    fn accept(&self, warehouse: &Warehouse, item: &Item) -> bool;
}

// Filtro de exemplo: rejeita itens com quantidade zero
struct NonZeroQuantityFilter;

impl ItemFilter for NonZeroQuantityFilter {
    fn accept(&self, _warehouse: &Warehouse, item: &Item) -> bool {
        item.quantity > 0
    }
}

// Filtro de exemplo: rejeita itens frágeis com data expirada
struct NonExpiredFilter {
    current_date: String,
}

impl NonExpiredFilter {
    fn new(current_date: String) -> Self {
        NonExpiredFilter { current_date }
    }
}

impl ItemFilter for NonExpiredFilter {
    fn accept(&self, _warehouse: &Warehouse, item: &Item) -> bool {
        if let Quality::Fragile { expiry_date, .. } = &item.quality {
            return expiry_date >= &self.current_date;
        }
        true // Não é um item frágil, então é aceito
    }
}

// IMPLEMENTAÇÃO DO ARMAZÉM

struct Warehouse {
    // Mapa de localização para item
    inventory: HashMap<Location, Item>,
    
    // Índices para busca eficiente
    id_index: HashMap<u32, HashSet<Location>>,
    name_index: HashMap<String, HashSet<Location>>,
    
    // Dimensões do armazém
    max_row: u32,
    max_shelf: u32,
    max_zone: u32,
    
    // Estratégia de alocação (usando Box para permitir polimorfismo)
    allocation_strategy: Box<dyn AllocationStrategy>,
    
    // Lista de filtros
    filters: Vec<Box<dyn ItemFilter>>,
}

impl Warehouse {
    fn new(max_row: u32, max_shelf: u32, max_zone: u32, strategy: Box<dyn AllocationStrategy>) -> Self {
        Warehouse {
            inventory: HashMap::new(),
            id_index: HashMap::new(),
            name_index: HashMap::new(),
            max_row,
            max_shelf,
            max_zone,
            allocation_strategy: strategy,
            filters: Vec::new(),
        }
    }
    
    // Adicionar um filtro
    fn add_filter(&mut self, filter: Box<dyn ItemFilter>) {
        self.filters.push(filter);
    }
    
    // Trocar estratégia de alocação
    fn set_allocation_strategy(&mut self, strategy: Box<dyn AllocationStrategy>) {
        self.allocation_strategy = strategy;
    }
    
    // Obter dimensões do armazém
    fn get_dimensions(&self) -> (u32, u32, u32) {
        (self.max_row, self.max_shelf, self.max_zone)
    }
    
    // Verificar se uma localização está livre
    fn is_location_free(&self, location: &Location) -> bool {
        !self.inventory.contains_key(location)
    }
    
    // Verificar se há zonas contíguas disponíveis para itens oversized
    fn has_consecutive_free_zones(&self, location: &Location, count: u32) -> bool {
        for i in 0..count {
            let loc = Location::new(location.row, location.shelf, location.zone + i);
            if !self.is_location_free(&loc) || loc.zone >= self.max_zone {
                return false;
            }
        }
        true
    }
    
    // Obter todas as localizações disponíveis
    fn get_available_locations(&self) -> Vec<Location> {
        let mut available = Vec::new();
        
        for row in 0..self.max_row {
            for shelf in 0..self.max_shelf {
                for zone in 0..self.max_zone {
                    let location = Location::new(row, shelf, zone);
                    if self.is_location_free(&location) {
                        available.push(location);
                    }
                }
            }
        }
        
        available
    }
    
    // Alocar um local para um novo item
    fn allocate_location(&self, item: &Item) -> Option<Location> {
        self.allocation_strategy.allocate(self, item)
    }
    
    // Verificar se um item é aceito pelos filtros
    fn is_item_accepted(&self, item: &Item) -> bool {
        for filter in &self.filters {
            if !filter.accept(self, item) {
                return false;
            }
        }
        true
    }
    
    // Adicionar um item ao armazém
    fn add_item(&mut self, item: Item) -> Result<Location, &'static str> {
        // Verificar se o item é aceito pelos filtros
        if !self.is_item_accepted(&item) {
            return Err("Item não aceito pelos filtros");
        }
        
        // Alocar um local para o item
        let location = match self.allocate_location(&item) {
            Some(loc) => loc,
            None => return Err("Não há localização disponível para este item"),
        };
        
        // Para itens oversized, reservar zonas adicionais
        if let Quality::Oversized { zones_needed } = item.quality {
            if zones_needed > 1 {
                // Verificar novamente se há espaço contíguo
                if !self.has_consecutive_free_zones(&location, zones_needed) {
                    return Err("Não há espaço contíguo suficiente");
                }
                
                // Marcar zonas adicionais como ocupadas (com o mesmo item)
                for i in 1..zones_needed {
                    let additional_loc = Location::new(
                        location.row, 
                        location.shelf, 
                        location.zone + i
                    );
                    self.inventory.insert(additional_loc.clone(), item.clone());
                    
                    // Atualizar índices
                    self.id_index.entry(item.id).or_default().insert(additional_loc.clone());
                    self.name_index.entry(item.name.clone()).or_default().insert(additional_loc);
                }
            }
        }
        
        // Adicionar o item no local principal
        self.inventory.insert(location.clone(), item.clone());
        
        // Atualizar índices
        self.id_index.entry(item.id).or_default().insert(location.clone());
        self.name_index.entry(item.name.clone()).or_default().insert(location.clone());
        
        Ok(location)
    }
    
    // Remover um item do armazém
    fn remove_item(&mut self, location: &Location) -> Result<Item, &'static str> {
        if let Some(item) = self.inventory.remove(location) {
            // Remover das estruturas de índice
            if let Some(locations) = self.id_index.get_mut(&item.id) {
                locations.remove(location);
                if locations.is_empty() {
                    self.id_index.remove(&item.id);
                }
            }
            
            if let Some(locations) = self.name_index.get_mut(&item.name) {
                locations.remove(location);
                if locations.is_empty() {
                    self.name_index.remove(&item.name);
                }
            }
            
            // Se for um item oversized, remover as zonas adicionais
            if let Quality::Oversized { zones_needed } = item.quality {
                for i in 1..zones_needed {
                    let additional_loc = Location::new(
                        location.row, 
                        location.shelf, 
                        location.zone + i
                    );
                    self.inventory.remove(&additional_loc);
                    
                    // Não é necessário atualizar índices aqui, pois todas as localizações
                    // já foram removidas do principal acima
                }
            }
            
            Ok(item)
        } else {
            Err("Localização vazia")
        }
    }
    
    // Buscar itens por ID
    fn find_by_id(&self, id: u32) -> (bool, usize) {
        if let Some(locations) = self.id_index.get(&id) {
            (true, locations.len())
        } else {
            (false, 0)
        }
    }
    
    // Buscar itens por nome
    fn find_by_name(&self, name: &str) -> (bool, usize) {
        if let Some(locations) = self.name_index.get(name) {
            (true, locations.len())
        } else {
            (false, 0)
        }
    }
    
    // Encontrar todas as localizações de um item por ID
    fn get_locations_by_id(&self, id: u32) -> Vec<Location> {
        if let Some(locations) = self.id_index.get(&id) {
            locations.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    // Listar todos os itens ordenados por nome
    fn list_all_items_sorted_by_name(&self) -> Vec<(&Item, &Location)> {
        let mut items = Vec::new();
        
        for (location, item) in &self.inventory {
            items.push((item, location));
        }
        
        // Ordenar por nome
        items.sort_by(|a, b| a.0.name.cmp(&b.0.name));
        
        items
    }
    
    // Verificar itens próximos da validade
    fn check_near_expiry(&self, current_date: &str) -> usize {
        let mut count = 0;
        let mut counted_ids = HashSet::new();
        
        for item in self.inventory.values() {
            // Evitar contar o mesmo item várias vezes (no caso de oversized)
            if counted_ids.contains(&item.id) {
                continue;
            }
            
            if item.is_near_expiry(current_date) {
                count += 1;
                counted_ids.insert(item.id);
            }
        }
        
        count
    }
}

// MAIN PARA DEMONSTRAÇÃO

fn main() {
    // Criar um armazém com estratégia de proximidade
    let mut warehouse = Warehouse::new(
        10, 10, 10, 
        Box::new(ProximityStrategy)
    );
    
    // Adicionar filtros
    warehouse.add_filter(Box::new(NonZeroQuantityFilter));
    warehouse.add_filter(Box::new(NonExpiredFilter::new("2025-05-01".to_string())));
    
    // Adicionar alguns itens de exemplo
    
    // Item normal
    let item1 = Item::new(
        1,
        "Caixa de Parafusos".to_string(),
        50,
        Quality::Normal,
    );
    
    // Item frágil
    let item2 = Item::new(
        2,
        "Vidro Temperado".to_string(),
        10,
        Quality::Fragile {
            expiry_date: "2025-05-30".to_string(),
            max_row: 3,
        },
    );
    
    // Item oversized
    let item3 = Item::new(
        3,
        "Porta de Madeira".to_string(),
        5,
        Quality::Oversized {
            zones_needed: 2,
        },
    );
    
    // Adicionar os itens ao armazém
    match warehouse.add_item(item1) {
        Ok(location) => println!("Item 1 adicionado em {}", location),
        Err(e) => println!("Erro ao adicionar item 1: {}", e),
    }
    
    match warehouse.add_item(item2) {
        Ok(location) => println!("Item 2 adicionado em {}", location),
        Err(e) => println!("Erro ao adicionar item 2: {}", e),
    }
    
    match warehouse.add_item(item3) {
        Ok(location) => println!("Item 3 adicionado em {}", location),
        Err(e) => println!("Erro ao adicionar item 3: {}", e),
    }
    
    // Mudar para estratégia round robin
    println!("\nMudando para estratégia Round Robin...");
    warehouse.set_allocation_strategy(Box::new(RoundRobinStrategy::new()));
    
    // Adicionar mais um item para demonstrar a estratégia round robin
    let item4 = Item::new(
        4,
        "Caixa de Ferramentas".to_string(),
        1,
        Quality::Normal,
    );
    
    match warehouse.add_item(item4) {
        Ok(location) => println!("Item 4 adicionado em {}", location),
        Err(e) => println!("Erro ao adicionar item 4: {}", e),
    }
    
    // Demonstrar busca por ID
    let (found, count) = warehouse.find_by_id(2);
    println!("\nBusca por ID 2: Encontrado = {}, Quantidade = {}", found, count);
    
    // Demonstrar busca por nome
    let (found, count) = warehouse.find_by_name("Porta de Madeira");
    println!("Busca por 'Porta de Madeira': Encontrado = {}, Quantidade = {}", found, count);
    
    // Demonstrar busca de localizações por ID
    let locations = warehouse.get_locations_by_id(3);
    println!("\nLocalizações do item com ID 3:");
    for loc in locations {
        println!("- {}", loc);
    }
    
    // Listar todos os itens ordenados por nome
    println!("\nTodos os itens ordenados por nome:");
    let items = warehouse.list_all_items_sorted_by_name();
    for (item, location) in items {
        println!("{} em {}", item, location);
    }
    
    // Verificar itens próximos da validade
    let near_expiry_count = warehouse.check_near_expiry("2025-05-28");
    println!("\nItens próximos da validade (a expirar em 3 dias): {}", near_expiry_count);
    
    // Remover um item
    println!("\nRemovendo o item da primeira localização...");
    let first_loc = Location::new(0, 0, 0);
    match warehouse.remove_item(&first_loc) {
        Ok(item) => println!("Item removido: {}", item),
        Err(e) => println!("Erro ao remover: {}", e),
    }
    
    // Listar novamente após remoção
    println!("\nItens após remoção:");
    let items = warehouse.list_all_items_sorted_by_name();
    for (item, location) in items {
        println!("{} em {}", item, location);
    }
}

