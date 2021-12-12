pub enum BFCommand {
    ShiftPointerLeft(),  // <
    ShiftPointerRight(), // >
    IncrementCell(),     // +
    DecrementCell(),     // -
    OutputByte(),        // .
    InputByte(),         // ,
    JumpForward(),       // [
    JumpBackward()       // ]
}

pub fn parse_string_into_commands(input: &String) -> Vec<BFCommand> {
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