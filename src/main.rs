mod bf_interpreter;
use bf_interpreter::BFInterpreter;

use std::{ env, fs, io::{ self, Write } };

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(file_name) = args.get(1) else {
        run_repl();
    };

    // The Brainf*ck source code
    let code = fs::read_to_string(file_name).unwrap();

    let mut interpreter = BFInterpreter::new(false);

    match interpreter.execute(&code) {
        Ok(()) => (),
        Err(e) => println!("\n{}", e)
    };
}

fn run_repl() -> ! {
    let mut interpreter = BFInterpreter::new(true);
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