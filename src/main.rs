fn main() {
    let input = String::from("++>+++++[<+>-]++++++++[<++++++>-]<.");

    let mut commands: Vec<char> = Vec::new();
    for c in input.chars() {
        if vec!('>', '<', '+', '-', '.', ',', '[', ']').contains(&c) {
            commands.push(c);
        }
    }

    let mut data_pointer: usize = 0;
    let mut memory: Vec<u8> = vec!();

    // Set the memory address specified by pointer to
    // equal new_value, expanding the memory stack if
    // it's too small
    /*let mut set_address = |pointer: &usize, new_val: &u8| {
        // Check if the memory stack is too short
        if &memory.len() <= &*pointer {
            // Extend the memory
            &memory.resize(pointer + 1, 0);
        }
        &mut memory[*pointer] = &mut *new_val;
    };*/

    /*let get_address = |pointer: &usize| -> u8 {
        match &memory.get(*pointer) {
            Some(x) => **x,
            None => 0
        }
    };*/

    for i in 0..commands.len() {
        println!("{}", commands[i]);
        match commands[i] {
            '>' => {
                data_pointer += 1;
            }
            '<' => {
                data_pointer -= 1;
            }
            '+' => {
                let current_value = get_address(&mut memory, &data_pointer);
                set_address(&mut memory, &data_pointer, &mut(current_value + 1));
            }
            '-' => {
                let current_value = get_address(&mut memory, &data_pointer);
                set_address(&mut memory, &data_pointer, &mut(current_value - 1));
            }
            '.' => {
                print!("{}", get_address(&mut memory, &data_pointer));
            }
            ',' => {
                //TODO: implement this
            }
            '[' => {}
            ']' => {}
            _ => {}
        }
    }

    println!("{:#?}", memory);
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