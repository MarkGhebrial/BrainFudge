use super::{
    BFCommand,
    BFMemory,
    BFError,
    BFErrorType
};
use std::io::{self, Write};

pub struct BFInterpreter {
    memory: BFMemory,
    debug_mode: bool
}

impl BFInterpreter {
    pub fn new() -> BFInterpreter {
        BFInterpreter {
            memory: BFMemory::new(),
            debug_mode: false
        }
    }

    pub fn new_with_debug() -> BFInterpreter {
        BFInterpreter {
            memory: BFMemory::new(),
            debug_mode: true
        }
    }

    /// Clear all the memory
    pub fn reset(&mut self) {
        self.memory = BFMemory::new();
    }

    pub fn execute(&mut self, bf_code: &String) -> Result<(), BFError> {
        let commands = BFInterpreter::parse_string_into_commands(bf_code);

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
                    //println!("TEST {}", self.memory.read_current_address());
                    print!("{}", char::from(self.memory.read_current_address()));
                    io::stdout().flush().unwrap();
                }
                BFCommand::InputByte() => {
                    //TODO: implement this
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
                            BFErrorType::UnmatchedBracket()
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
                        BFErrorType::UnmatchedBracket()
                    ));
                }
            }
            i += 1;
        }
        if self.debug_mode {
            println!("{}", self.memory);
        }
        Ok(())
    }

    pub fn print_memory(&self) {
        println!("{}", self.memory);
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

    fn parse_commands_into_string(input: &Vec<BFCommand>) -> String {
        let mut out = String::new();

        for c in input {
            match c {
                BFCommand::ShiftPointerLeft() => { out += "<"; },
                BFCommand::ShiftPointerRight() => { out += ">"; },
                BFCommand::IncrementCell() => { out += "+"; },
                BFCommand::DecrementCell() => { out += "-"; },
                BFCommand::OutputByte() => { out += "."; },
                BFCommand::InputByte() => { out += ","; },
                BFCommand::JumpForward() => { out += "["; },
                BFCommand::JumpBackward() => { out += "]"; },
            }
        }
        out
    }
}

/// Ensure that the interpterer runs Brainf**k code correctly
#[test]
fn validation() {
    let mut interpreter = BFInterpreter::new();

    match interpreter.execute(&String::from("+++>+++++[<+>-]")) {
        Err(e) => panic!("Code that should execute failed\n{}", e),
        Ok(()) => {}
    };
    assert_eq!(interpreter.memory.read_address(&0), 8);
}

#[test]
fn command_parser() {
    assert_eq!(
        BFInterpreter::parse_commands_into_string(&BFInterpreter::parse_string_into_commands(&String::from("><+-[],."))),
        String::from("><+-[],.")
    );
}