pub enum BFErrorType {
    UnmatchedBracket(),
    PointerOutOfBounds(),
    CellOverflow()
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
        let message = String::from(match self.err_type {
            BFErrorType::UnmatchedBracket() => "Unmatched bracket",
            BFErrorType::PointerOutOfBounds() => "Pointer moved out bounds",
            BFErrorType::CellOverflow() => "Cell exceeding 8-bit limit"
        });
        write!(f, "{}\n{}^ Err: {}", self.erroneous_code, spaces, message)
    }
}