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