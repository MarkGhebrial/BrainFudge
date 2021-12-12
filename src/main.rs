mod interpreter;
use interpreter::{
    BFCommand, 
    parse_string_into_commands, 
    BFMemory
};

fn main() {
    // The Brainf**k source code
    let input = String::from("+++>+++++[<+>-]++++++++[<++++++>-]<.");
    //let input = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");

    let commands: Vec<BFCommand> = parse_string_into_commands(&input);

    let mut memory = BFMemory::new();

    let mut i = 0;
    'main_loop: while i < commands.len() {
        match commands[i] {
            BFCommand::ShiftPointerRight() => {
                // Move the pointer forward
                memory.increment_pointer(&1);
            }
            BFCommand::ShiftPointerLeft() => {
                // Move it backward
                memory.decrement_pointer(&1);
            }
            BFCommand::IncrementCell() => {
                // Increment the currently pointed byte by one
                memory.increment_current_address(&1);
            }
            BFCommand::DecrementCell() => {
                // Decrement the currently pointed byte by one
                memory.decrement_current_address(&1);
            }
            BFCommand::OutputByte() => {
                // Convert the pointed byte to ASCII and print it
                print!("{}", char::from(memory.read_current_address()));
            }
            BFCommand::InputByte() => {
                //TODO: implement this
            }
            BFCommand::JumpForward() => {
                // Jump foward to the next ']' if the current byte is non-zero
                if memory.read_current_address() == 0 {
                    for j in i..commands.len() {
                        match commands[j] {
                            BFCommand::JumpBackward() => {
                                i = j + 1;
                                continue 'main_loop;
                            },
                            _ => {}
                        };
                    }
                    panic!("Unmatched '[' at position {}", i);
                }
            }
            BFCommand::JumpBackward() => {
                // Jump back to the previous '['
                for j in (0..i).rev() {
                    match commands[j] {
                        BFCommand::JumpForward() => {
                            i = j;
                            continue 'main_loop;
                        },
                        _ => {}
                    };
                }
                panic!("Unmatched '[' at position {}", i);
            }
        }
        i += 1;
    }
}