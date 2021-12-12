mod commands;
pub use commands::{
    BFCommand, 
    parse_string_into_commands
};

mod mem;
pub use mem::BFMemory;

mod interpreter;
pub use interpreter::{
    BFInterpreter,
    UnmatchedBracketError
};

// TODO: Get my tests to run correctly!
#[cfg(tests)]
mod tests {
    #[test]
    fn unmatched_bracket_err_display() {
        let e = UnmatchedBracketError::new(17, String::from("+++>+++++[<+>-]++]++++++[<++++++>-]<."));

        assert_eq!(
            format!("{}", e),
            String::from("+++>+++++[<+>-]++]++++++[<++++++>-]<.\n                 ^ Unmatched bracket")    
        )
    }

    /*
    TODO: Implement tests for:
        - BFInterpreter.execute()
        - BFInterpreter::parse_string_into_commands and BFInterpreter::parse_commands_into_string
        - BFMemory and its functions
    */
}