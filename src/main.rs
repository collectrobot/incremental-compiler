mod lexer;
mod token;
mod ast;
mod parser;
mod io;
mod interp;
mod intermediate;
mod uniquify;

use io::{get_line};
use lexer::{Lexer};
use parser::{Parser};
use interp::{Interpreter};
use uniquify::{uniquify_program};

#[derive(PartialEq)]
enum ReplResult {
    KeepGoing,
    Stop
}  

fn print_grammer() {
println!("
expr  ::= int | (read) | ('-' exp) | ('+' exp exp)
        | var | (let ([var exp]+) exp)
rlang ::= exp
");
}

fn print_commands() {
println!("
:grammer     print the grammer
:quit        quit the repl
");
}

fn handle_repl_command(command: &str) -> ReplResult {
    match command {
        ":help" => { print_commands(); ReplResult::KeepGoing },
        ":grammer" => { print_grammer(); ReplResult::KeepGoing },
        ":quit" => ReplResult::Stop,
        _ => ReplResult::KeepGoing
    }
}

fn main() -> std::io::Result<()> {

    let mut input: String;
    'repl_loop:loop {

        input = get_line();

        if input == "" {
            continue 'repl_loop;
        }

        if input.starts_with(":") {
            if handle_repl_command(&input) == ReplResult::Stop {
                println!("Goodbye!");
                break 'repl_loop;
            }

            continue 'repl_loop;
        }

        let mut l = Lexer::new(&input);

        let tokens = l.lex();

        let mut p = Parser::new(tokens.clone());

        let program = p.parse();

        let uniquify_pass = uniquify_program(program);

        let mut interp = Interpreter::new();

        let result = interp.interpret(uniquify_pass);

        let result = 
            match result {
                Ok(n) => n.to_string(),
                Err(err) => err
            };

        println!("result: {}", result);

        /*
        for token in &tokens {
            println!("{:?}", token);
        }
        */
    }

    Ok(())
}