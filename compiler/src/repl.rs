#![allow(unused_imports)]

use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};
use crate::frontend::partial_eval::{partially_evaluate};
use crate::ir::explicate::{explicate_control};
use crate::backend::x64_backend::{IRToX64Transformer};
use crate::interpreter::{
    Interpreter, 
    CachedRuntimeCall,
    CachedFunctionResult,
    interp_ast::AstInterpreter,
    interp_ir::IrInterpreter,
};

use crate::io::{get_line};

#[derive(PartialEq)]
enum ReplResult {
    KeepGoing,
    Stop
}  

#[derive(Copy, Clone)]
struct ReplCommand {
    pub cmd: &'static str,
    pub help: &'static str,
    pub action: fn (&mut Repl) -> ReplResult,
}

pub struct Repl {
    commands: Vec<ReplCommand>,
    show_ast: bool,
    show_ir: bool,
    show_x64: bool,
}

impl Repl {

    pub fn new() -> Self {
        let commands = vec!(
            ReplCommand { cmd: ":help", help: "show available commands", action: Repl::print_help },
            ReplCommand {
                cmd: ":show-ast",
                help: "show the abstract syntax tree",
                action: |r| {
                    if r.show_ast == true {
                        r.show_ast = false;
                    } else {
                        r.show_ast = true;
                    }

                    ReplResult::KeepGoing
                },
            },
            ReplCommand {
                cmd: ":show-ir",
                help: "show the intermediate representation",
                action: |r| {
                    if r.show_ir == true {
                        r.show_ir = false;
                    } else {
                        r.show_ir = true;
                    }

                    ReplResult::KeepGoing
                },
            },
            ReplCommand {
                cmd: ":show-x64",
                help: "show the x64 representation",
                action: |r| {
                    if r.show_x64 == true {
                        r.show_x64 = false;
                    } else {
                        r.show_x64 = true;
                    }

                    ReplResult::KeepGoing
                },
            },
            ReplCommand { cmd: ":grammer", help: "print the grammer", action: Repl::print_grammer },
            ReplCommand { cmd: ":quit", help: "exit the repl", action: Repl::quit }
        );

        Repl {
            commands: commands,
            show_ast: false,
            show_ir: false,
            show_x64: false,
        }
    }

    fn print_grammer(&mut self) -> ReplResult {
        println!("
expr  ::= int | (read) | ('-' exp) | ('+' exp exp)
        | var | (let ([var exp]+) exp)
rlang ::= exp
        ");

        ReplResult::KeepGoing
    }

    fn quit(&mut self) -> ReplResult {
        ReplResult::Stop
    }

    fn print_help(&mut self) -> ReplResult {

        println!("");
        for cmd in &self.commands {
            println!("{} - {}", cmd.cmd, cmd.help);
        }
        println!("");

        ReplResult::KeepGoing
    }

    fn handle_repl_command(&mut self, command: &str) -> ReplResult {

        let cmd = self.commands.iter().position(|&c| c.cmd == command);

        match cmd {
            Some(index) => {
                let repl_cmd = self.commands[index];
                let action = repl_cmd.action;
                action(self)
            },

            _ => ReplResult::KeepGoing
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut input: String;
        'repl_loop:loop {

            input = get_line();

            if input == "" {
                continue 'repl_loop;
            }

            if input.starts_with(":") {
                if self.handle_repl_command(&input) == ReplResult::Stop {
                    println!("Goodbye!");
                    break 'repl_loop;
                }

                continue 'repl_loop;
            }

            let mut l = Lexer::new(&input);

            let tokens = l.lex();

            let mut p = Parser::new(tokens.clone());

            let program = p.parse();

            if !p.parse_success() {
                p.print_errors();
                continue 'repl_loop;
            }

            let uniquified_program = uniquify_program(program);

            let partially_evaluated_program = partially_evaluate(uniquified_program);

            let decomplified_program = decomplify_program(partially_evaluated_program);

            if self.show_ast {
                println!("AST:");
                println!("{:#?}", decomplified_program);
            }

            let mut runtime_cache = CachedRuntimeCall::new();

            {
                let mut ast_interpreter = AstInterpreter::new(decomplified_program.clone(), &mut runtime_cache);

                let mut interpreter = Interpreter::new(&mut ast_interpreter);

                let result = interpreter.run();

                if result.had_error {
                    for error in result.errors {
                        println!("{}", error);
                    }
                    continue 'repl_loop;
                } else {
                    println!("Result of interpreting the AST: {}\n", result.value.unwrap());
                }
            }

            let intermediate_repr = explicate_control(decomplified_program);

            if self.show_ir {
                println!("IR:");
                println!("{:#?}", intermediate_repr);
            }

            runtime_cache.do_write(false);

            {
                let mut ir_interpreter = IrInterpreter::new(intermediate_repr.clone(), &mut runtime_cache);
                let mut interpreter = Interpreter::new(&mut ir_interpreter);

                let result = interpreter.run();

                if result.had_error {
                    for error in result.errors {
                        println!("{}", error);
                    }
                    continue 'repl_loop;
                } else {
                    println!("Result of interpreting the IR: {}\n", result.value.unwrap());
                }
            }

            let x64prog =
                IRToX64Transformer::new(intermediate_repr)
                .transform();
  
            if self.show_x64 {
                println!("{:#?}", x64prog);
            }

        }

        Ok(())
    }
}


