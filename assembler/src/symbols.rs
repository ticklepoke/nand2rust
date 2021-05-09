use std::collections::{HashMap};

static RESERVED_SYMBOLS: &[(&str, usize)] = &[
    ("SP", 0),
    ("LCL", 1),
    ("ARG", 2),
    ("THIS", 3),
    ("THAT", 4),
    ("R0", 0),
    ("R1", 1),
    ("R2", 2),
    ("R3", 3),
    ("R4", 4),
    ("R5", 5),
    ("R6", 6),
    ("R7", 7),
    ("R8", 8),
    ("R9", 9),
    ("R10", 10),
    ("R11", 11),
    ("R12", 12),
    ("R13", 13),
    ("R14", 14),
    ("R15", 15),
    ("SCREEN", 16384),
    ("KBD", 24576),
];

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut symbol_table = SymbolTable {
            symbols: HashMap::new(),
        };

        for (symbol, address) in RESERVED_SYMBOLS {
            symbol_table.add_entity(symbol, *address).unwrap();
        }
        symbol_table
    }

    pub fn add_entity(&mut self, symbol: &str, address: usize) -> Result<(), ()> {
        if !&self.contains(symbol) {
            &self.symbols.insert(symbol.to_string(), address);
            return Ok(());
        }
        Err(())
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.symbols.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<usize> {
        if let Some(address) = self.symbols.get(symbol) {
            return Some(*address);
        } else {
            return None;
        }
    }
}
