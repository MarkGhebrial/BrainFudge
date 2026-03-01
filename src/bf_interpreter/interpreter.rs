use super::{
    BFCommand,
    BFMemory,
    BFError,
    BFErrorType
};
use std::io::{self, Read, Write};

pub struct BFInterpreter<R, W> {
    memory: BFMemory,
    debug_mode: bool,

    read: R,
    write: W,
}

impl BFInterpreter<std::io::Stdin, std::io::Stdout> {
    pub fn new(debug_mode: bool) -> Self {
        Self {
            memory: BFMemory::new(),
            debug_mode,
            read: std::io::stdin(),
            write: std::io::stdout(),
        }
    }
}

impl<R, W> BFInterpreter<R, W> where R: Read, W: Write {
    pub fn from_rw(read: R, write: W, debug_mode: bool) -> Self {
        Self {
            memory: BFMemory::new(),
            debug_mode,
            read,
            write,
        }
    }

    /// Clear all the memory
    pub fn reset(&mut self) {
        self.memory = BFMemory::new();
    }

    pub fn execute(&mut self, bf_code: &String) -> Result<(), BFError> {
        let commands = parse_string_into_commands(bf_code);

        // Process the commands (blocks the current thread)
        let mut i = 0;
        'main_loop: while i < commands.len() {
            match commands[i] {
                BFCommand::ShiftPointerRight() => {
                    // Move the pointer forward
                    self.memory.increment_pointer(&1);
                }
                BFCommand::ShiftPointerLeft() => {
                    // Move it backward
                    if let Err(err) = self.memory.decrement_pointer(&1) {
                        return Err(BFError::new(
                            i,
                            bf_code.to_string(),
                            err
                        ));
                    }
                }
                BFCommand::IncrementCell() => {
                    // Increment the currently pointed byte by one
                    if let Err(err) = self.memory.increment_current_address(&1) {
                        return Err(BFError::new(
                            i,
                            bf_code.to_string(),
                            err
                        ));
                    }
                }
                BFCommand::DecrementCell() => {
                    // Decrement the currently pointed byte by one
                    if let Err(err) = self.memory.decrement_current_address(&1) {
                        return Err(BFError::new(
                            i,
                            bf_code.to_string(),
                            err
                        ));
                    }
                }
                BFCommand::OutputByte() => {
                    // Convert the pointed byte to ASCII and print it
                    self.write.write(&[self.memory.read_current_address()]).unwrap();
                    io::stdout().flush().unwrap();
                }
                BFCommand::InputByte() => {
                    let mut buf: [u8; 1] = [0];
                    self.read.read_exact(&mut buf).unwrap();
                    self.memory.write_to_current_address(&buf[0]);
                }
                BFCommand::JumpForward() => {
                    // Jump foward to the next ']' if the current byte is non-zero
                    if self.memory.read_current_address() == 0 {
                        let mut nesting_level = 1;
                        for j in i+1..commands.len() {
                            match commands[j] {
                                BFCommand::JumpForward() => nesting_level += 1,
                                BFCommand::JumpBackward() => {
                                    nesting_level -= 1;
                                    if nesting_level == 0 {
                                        i = j + 1;
                                        continue 'main_loop;
                                    }
                                },
                                _ => {}
                            };
                        }
                        return Err(BFError::new(
                            i, 
                            bf_code.to_string(),
                            BFErrorType::UnmatchedBracket
                        ));
                    }
                }
                BFCommand::JumpBackward() => {
                    // Jump back to the previous '['
                    let mut nesting_level = 1;
                    for j in (0..i).rev() {
                        match commands[j] {
                            BFCommand::JumpBackward() => nesting_level += 1,
                            BFCommand::JumpForward() => {
                                nesting_level -= 1;
                                if nesting_level == 0 {
                                    i = j;
                                    continue 'main_loop;
                                }
                            },
                            _ => {}
                        };
                    }
                    return Err(BFError::new(
                        i, 
                        bf_code.to_string(),
                        BFErrorType::UnmatchedBracket
                    ));
                }
            }
            i += 1;
        }
        if self.debug_mode {
            writeln!(self.write, "{}", self.memory);
        }
        Ok(())
    }

    pub fn print_memory(&self) {
        println!("{}", self.memory);
    }
}

fn parse_string_into_commands(input: &String) -> Vec<BFCommand> {
    let mut out: Vec<BFCommand> = vec!();
    
    for c in input.chars() {
        let command: Option<BFCommand> = match c {
            '<' => Some(BFCommand::ShiftPointerLeft()),
            '>' => Some(BFCommand::ShiftPointerRight()),
            '+' => Some(BFCommand::IncrementCell()),
            '-' => Some(BFCommand::DecrementCell()),
            '.' => Some(BFCommand::OutputByte()),
            ',' => Some(BFCommand::InputByte()),
            '[' => Some(BFCommand::JumpForward()),
            ']' => Some(BFCommand::JumpBackward()),
            _ => None
        };
        match command {
            Some(g) => { out.push(g); }
            None => {}
        };
    }
    out
}

/// Ensure that the interpterer runs Brainf**k code correctly
#[test]
fn validation() {
    let mut interpreter = BFInterpreter::new(false);

    match interpreter.execute(&String::from("+++>+++++[<+>-]")) {
        Err(e) => panic!("Code that should execute failed\n{}", e),
        Ok(()) => {}
    };
    assert_eq!(interpreter.memory.read_address(&0), 8);
}
