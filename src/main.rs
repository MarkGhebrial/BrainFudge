mod bf_interpreter;
use bf_interpreter::{
    BFInterpreter
};

fn main() {
    // The Brainf**k source code
    let input = String::from("+++>+++++[<+>-]++++++++[<++++++>-]<.");
    //let input = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");

    let mut interpreter = BFInterpreter::new();

    print!("\n");
    match interpreter.execute(&input) {
        Ok(()) => println!("Huzzah! The program executed with no errors"),
        Err(e) => println!("{}", e)
    };
}