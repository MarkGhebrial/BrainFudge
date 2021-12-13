pub struct BFMemory {
    memory: Vec<u8>,
    pointer: usize
}

#[allow(dead_code)]
impl BFMemory {
    pub fn new() -> BFMemory {
        BFMemory {
            memory: vec!(0),
            pointer: 0
        }
    }

    pub fn set_pointer(&mut self, val: &usize) {
        self.pointer = *val;
    }

    pub fn get_pointer(&mut self) -> usize { 
        self.pointer
    }

    pub fn increment_pointer(&mut self, amount: &usize) {
        self.pointer += amount;
    }

    pub fn decrement_pointer(&mut self, amount: &usize) {
        self.pointer -= amount;
    }

    pub fn write_to_address(&mut self, address: &usize, new_val: &u8) {
        let val = self.memory.get_mut(*address);
        match val {
            Some(x) => *x = *new_val,
            None => {
                self.memory.resize(address + 1, 0);
                *self.memory.get_mut(*address).unwrap() = *new_val;
            }
        }
    }

    pub fn write_to_current_address(&mut self, new_val: &u8) {
        let address = self.pointer;
        self.write_to_address(&address, new_val);
    }

    pub fn read_address(&mut self, address: &usize) -> u8 {
        let val = self.memory.get(*address);
        match val {
            Some(x) => *x,
            None => { 
                // If the memory is not long enough, lengthen it
                self.memory.resize(address + 1, 0);
                *self.memory.get(*address).unwrap()
            }
        }
    }

    pub fn read_current_address(&mut self) -> u8 {
        let address = self.pointer;
        self.read_address(&address)
    }

    pub fn increment_current_address(&mut self, amount: &u8) {
        let new_val = self.read_current_address() + amount;
        self.write_to_current_address(&new_val);
    }

    pub fn decrement_current_address(&mut self, amount: &u8) {
        let new_val = self.read_current_address() - amount;
        self.write_to_current_address(&new_val);
    }
}

impl Iterator for BFMemory {
    type Item = u8;

    fn next(&mut self) -> Self::Item {
        
    }
}

use std::fmt;
impl fmt::Display for BFMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first_line = String::new();
        let mut second_line = String:: new();

        for
    }
}

/*
[0][255][57][100][]
0  1    2   3^
*/