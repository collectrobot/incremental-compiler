mod lexer;
mod token;
mod ast;
mod parser;
mod io;
mod interp;
mod uniquify;
mod decomplify;
mod explicate;

#[macro_use]
mod utility;

use io::{get_line};
use lexer::{Lexer};
use parser::{Parser};
use interp::{Interpreter};
use uniquify::{uniquify_program};
use decomplify::{decomplify_program};
use explicate::{explicate_control};

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

        let mut program = p.parse();

        if !p.parse_success() {
            p.print_errors();
            continue 'repl_loop;
        }

        program = uniquify_program(program);

        let for_ir = program.clone();
        let ast_interpret = decomplify_program(program);

        println!("{:#?}", ast_interpret);

        let mut interp = Interpreter::new(ast_interpret);

        let result = interp.interpret();

        if interp.had_error() {
            interp.print_errors();
            continue 'repl_loop;
        }

        let result = 
            match result {
                Ok(n) => n.to_string(),
                Err(err) => err
            };

        //println!("result: {}", result);

        let intermediate_repr = explicate_control(for_ir);

        println!("{:#?}", intermediate_repr);


        /*
        for token in &tokens {
            println!("{:?}", token);
        }
        */
    }

    Ok(())
}