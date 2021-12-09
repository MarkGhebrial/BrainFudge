fn main() {
    // The Brainf*** source code
    let input = String::from("++>+++++[<+>-]++++++++[<++++++>-]<.");

    // Parse each symbol of the source code
    let mut commands: Vec<char> = Vec::new();
    for c in input.chars() {
        if vec!('>', '<', '+', '-', '.', ',', '[', ']').contains(&c) {
            commands.push(c);
        }
    }

    let mut data_pointer: usize = 0;
    let mut memory: Vec<u8> = vec!();

    let mut i = 0;
    'main_loop: while i < commands.len() {
        match commands[i] {
            '>' => {
                // Move the pointer forward
                data_pointer += 1;
            }
            '<' => {
                // Move it backward
                data_pointer -= 1;
            }
            '+' => {
                // Increment the currently pointed byte by one
                let current_value = get_address(&mut memory, &data_pointer);
                set_address(&mut memory, &data_pointer, &mut(current_value + 1));
            }
            '-' => {
                // Decrement the currently pointed byte by one
                let current_value = get_address(&mut memory, &data_pointer);
                set_address(&mut memory, &data_pointer, &mut(current_value - 1));
            }
            '.' => {
                // Convert the pointed byte to ASCII and print it
                print!("{}", char::from(get_address(&mut memory, &data_pointer)));
            }
            ',' => {
                //TODO: implement this
            }
            '[' => {
                // Jump foward to the next ']' if the current byte is non-zero
                if get_address(&mut memory, &data_pointer) == 0 {
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

fn set_address (memory: &mut Vec<u8>, pointer: &usize, new_val: &mut u8) {
    let val = memory.get_mut(*pointer);
    match val {
        Some(x) => *x = *new_val,
        None => {
            memory.resize(pointer + 1, 0);
            *memory.get_mut(*pointer).unwrap() = *new_val;
        }
    }
}

fn get_address (memory: &mut Vec<u8>, pointer: &usize) -> u8 {
    let val = memory.get(*pointer);
    match val {
        Some(x) => *x,
        None => { 
            // If the memory is not long enough, lengthen it
            memory.resize(pointer + 1, 0);
            *memory.get(*pointer).unwrap()
        }
    }
}