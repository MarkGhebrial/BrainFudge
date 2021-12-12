mod commands;
pub use commands::{BFCommand, parse_string_into_commands};

mod mem;
pub use mem::BFMemory;