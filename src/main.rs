mod mem;
use mem::BFMemory;

fn main() {
    // The Brainf*** source code
    let input = String::from("++>+++++[<+>-]++++++++[<++++++>-]<.");
    //let input = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");

    // Parse each symbol of the source code
    let mut commands: Vec<char> = Vec::new();
    for c in input.chars() {
        if vec!('>', '<', '+', '-', '.', ',', '[', ']').contains(&c) {
            commands.push(c);
        }
    }

    let mut memory = BFMemory::new();

    let mut i = 0;
    'main_loop: while i < commands.len() {
        match commands[i] {
            '>' => {
                // Move the pointer forward
                memory.increment_pointer(&1);
            }
            '<' => {
                // Move it backward
                memory.decrement_pointer(&1);
            }
            '+' => {
                // Increment the currently pointed byte by one
                memory.increment_current_address(&1);
            }
            '-' => {
                // Decrement the currently pointed byte by one
                memory.decrement_current_address(&1);
            }
            '.' => {
                // Convert the pointed byte to ASCII and print it
                print!("{}", char::from(memory.read_current_address()));
            }
            ',' => {
                //TODO: implement this
            }
            '[' => {
                // Jump foward to the next ']' if the current byte is non-zero
                if memory.read_current_address() == 0 {
                    for j in i..commands.len() {
                        if commands[j] == ']' {
                            i = j + 1;
                            continue 'main_loop; // Skip incrementing the current command
                        }
                    }
                    panic!("Unmatched '[' at position {}", i);
                }
            }
            ']' => {
                // Jump back to the previous '['
                for j in (0..i).rev() {
                    if commands[j] == '[' { // Move backwards through the commands
                        i = j;
                        continue 'main_loop;
                    }
                }
                panic!("Unmatched '[' at position {}", i);
            }
            _ => {} // Cover all cases
        }
        i += 1;
    }
}