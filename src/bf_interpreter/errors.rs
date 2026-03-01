pub enum BFErrorType {
    UnmatchedBracket(),
    PointerOutOfBounds(),
    CellOverflow {
        cell_index: usize,
        cell_value: isize,
    }
}

/// An instance of this struct will be returned in a Result:Err() if the
/// interpreter is given Brainf**k code that contains unmatched bracket
/// operators
pub struct BFError {
    location: usize,
    erroneous_code: String,
    err_type: BFErrorType
}

impl BFError {
    pub fn new(location: usize, erroneous_code: String, err_type: BFErrorType) -> BFError {
        BFError { location, erroneous_code, err_type }
    }
}

use std::fmt;
impl fmt::Display for BFError {
    /// Display something like this:
    /// +++>+++++[<+>-]++]++++++[<++++++>-]<.
    ///                  ^ Err: Unmatched bracket
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut spaces = String::new();
        for _i in 0..self.location {
            spaces += " ";
        }
        let message: String = match self.err_type {
            BFErrorType::UnmatchedBracket() => "Unmatched bracket".to_owned(),
            BFErrorType::PointerOutOfBounds() => "Pointer moved out bounds".to_owned(),
            BFErrorType::CellOverflow { cell_index, cell_value } => format!("Cell #{cell_index}={cell_value} exceeding 8-bit limit")
        };
        write!(f, "{}\n{}^ Err: {}", self.erroneous_code.trim(), spaces, message)
    }
}