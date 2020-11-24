use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, u16>,
    counter: u16,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let table: HashMap<String, u16> = [
            ("SP", 0),
            ("LCL", 1),
            ("ARG", 2),
            ("THIS", 3),
            ("THAT", 4),
            ("r0", 0),
            ("R0", 0),
            ("r1", 1),
            ("R1", 1),
            ("r2", 2),
            ("R2", 2),
            ("r3", 3),
            ("R3", 3),
            ("r4", 4),
            ("R4", 4),
            ("r5", 5),
            ("R5", 5),
            ("r6", 6),
            ("R6", 6),
            ("r7", 7),
            ("R7", 7),
            ("r8", 8),
            ("R8", 8),
            ("r9", 9),
            ("R9", 9),
            ("r10", 10),
            ("R10", 10),
            ("r11", 11),
            ("R11", 11),
            ("r12", 12),
            ("R12", 12),
            ("r13", 13),
            ("R13", 13),
            ("r14", 14),
            ("R14", 14),
            ("r15", 15),
            ("R15", 15),
            ("SCREEN", 16834),
            ("KBD", 24576),
        ]
        .iter()
        .map(|(key, val)| (key.to_owned().to_string(), *val))
        .collect();

        SymbolTable { table, counter: 16 }
    }

    pub fn add_entry(&mut self, symbol: String, addr: u16) {
        self.table.insert(symbol, addr);
    }

    pub fn add_variable(&mut self, symbol: String) -> u16 {
        self.table.insert(symbol, self.counter);
        self.counter += 1;
        self.counter - 1
    }

    pub fn get_addr(&self, symbol: &String) -> Option<u16> {
        match self.table.get(symbol) {
            Some(val) => Some(*val),
            None => None,
        }
    }

    pub fn contains(&self, symbol: String) -> bool {
        self.table.get(&symbol).is_some()
    }

}
