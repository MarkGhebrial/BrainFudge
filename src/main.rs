mod bf_interpreter;
use bf_interpreter::{
    BFInterpreter
};

use std::{ env, io::{ self, Write } };

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_repl();
    }

    // The Brainf**k source code
    //let input = String::from("+++>+++++[<+>-]++++++++[<++++++>-]<.");
    let input = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");


    let mut interpreter = BFInterpreter::new();

    match interpreter.execute(&input) {
        Ok(()) => println!("\nHuzzah! The program executed with no errors"),
        Err(e) => println!("\n{}", e)
    };
}

fn run_repl() {
    let mut interpreter = BFInterpreter::new_with_debug();
    loop {
        // Probably not the best choice for this repl
        // because ">>>" is valid Brainf**k code and
        // therefore might be a tad confusing
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == String::from("clear") {
            interpreter.reset();
            interpreter.print_memory();
            continue;
        }

        match interpreter.execute(&input) {
            Ok(()) => {},
            Err(e) => println!("{}", e)
        };
    }
}