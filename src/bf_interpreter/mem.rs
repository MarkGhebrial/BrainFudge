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
        self.read_address(val); // Extend the array
    }

    pub fn get_pointer(&self) -> usize { 
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

    /// Read the byte at the specified address, resizing the
    /// memory if it is too small
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

    /// Read the byte at the specified address, but don't resize
    /// it.
    ///
    /// Useful when self is not mutable
    pub fn read_address_no_resize (&self, address: &usize) -> u8 {
        let val = self.memory.get(*address);
        *val.unwrap() // Panic if index is out-of-bounds
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

    pub fn len(&self) -> usize {
        self.memory.len()
    }
}

use std::fmt;
impl fmt::Display for BFMemory {
    /// Display the state of the memory in a table like this:
    ///
    /// Memory address | 0 | 1   | 2  [ 3   ] 4 |
    ///      Byte data | 0 | 255 | 57 [ 100 ] 5 |
    /// ASCII encoding |   | 0   | D  [ Q   ] 2 |
    ///
    /// The column with square brackets is the memory cell that is currently
    /// being pointed to. 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // These strings represent the first, second, and third, lines
        // output respectively
        let mut first_line = String::from("Memory address ");
        let mut second_line = String::from("     Byte data ");
        let mut third_line = String::from("ASCII encoding ");

        // Iterate through every cell in the memory
        for i in 0..self.len() {
            let bracket: &str; // The opening bracket

            // Check if the cell being iterated upon is referenced by the pointer
            if self.get_pointer() == i {
                bracket = "[ "; // If so, then use square brackets insted of pipes
            } else {
                if self.get_pointer() == i - i { // If the last cell is the referenced one
                    bracket = "] "
                } else {
                    bracket = "| ";
                }
            }
            first_line += bracket;
            second_line += bracket;
            third_line += bracket;

            // Parse the cell's address and contents into strings
            let mut address = i.to_string();
            let mut byte_data = self.read_address_no_resize(&i).to_string();

            let ascii_char = char::from(self.read_address_no_resize(&i));
            let mut ascii_encoding = String::new();
            if ascii_char.is_ascii_graphic() { // Make sure it's a graphical character (as to not mess up the spacing)
                ascii_encoding += &String::from(ascii_char);
            }

            // The width of each cell in the final output (not memory)
            let cell_width = std::cmp::max(address.len(), byte_data.len());

            // This closure pads a string with spaces to make it the correct length
            let pad = | s: &mut String | {
                while s.len() < cell_width {
                    *s += " ";
                }
            };
            pad(&mut address);
            pad(&mut byte_data);
            pad(&mut ascii_encoding);

            // At this point, address, byte_data, and ascii_encoding should look like this:
            // "| <value>", "[ <value>", or "] <value>"

            // Append the column to the output 
            first_line += &format!("{} ", address);
            second_line += &format!("{} ", byte_data);
            third_line += &format!("{} ", ascii_encoding);
        }

        // Add the closing brackets to the last column of the table
        let bracket: &str;
        if self.get_pointer() == self.len() - 1 {
            bracket = "]"
        } else {
            bracket = "|"
        }
        first_line += bracket;
        second_line += bracket;
        third_line += bracket;

        write!(f, "{}\n{}\n{}", first_line, second_line, third_line)
    }
}

/*
Memory address | 0 | 1   | 2  [ 3   ] 4 |
     Byte data | 0 | 255 | 57 [ 100 ] 5 |
ASCII encoding | a | 0   | D  [ Q   ] 2 |

+++>+++++[<+>-]++++++++[<++++++>-]<.
                ^ Current command
*/