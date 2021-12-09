fn main() {
    let input = String::from("++>+++++[<+>-]++++++++[<++++++>-]<.");

    let mut commands: Vec<char> = Vec::new();
    for c in input.chars() {
        commands.push(c);
    }

    let mut data_pointer: usize = 0;
    let mut memory: Vec<u8> = vec!();
    let mut set_address = |pointer, new_val: u8| {
        // Check if the memory stack is too short
        if memory.len() <= pointer {
            // Extend the memory
            memory.resize(pointer + 1, 0);
        }
        memory[pointer] = new_val
    };

    data_pointer = 1;
    set_address(data_pointer, 255);

    println!("{:#?}", memory);
}