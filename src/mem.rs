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

    pub fn write_to_current_address(&mut self, new_val: &u8) {
        let val = self.memory.get_mut(self.pointer);
        match val {
            Some(x) => *x = *new_val,
            None => {
                self.memory.resize(self.pointer + 1, 0);
                *self.memory.get_mut(self.pointer).unwrap() = *new_val;
            }
        }
    }

    pub fn read_current_address(&mut self) -> u8 {
        let val = self.memory.get(self.pointer);
        match val {
            Some(x) => *x,
            None => { 
                // If the memory is not long enough, lengthen it
                self.memory.resize(self.pointer + 1, 0);
                *self.memory.get(self.pointer).unwrap()
            }
        }
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