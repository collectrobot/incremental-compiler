#![allow(unused_imports)]

use runtime::types::{RuntimeI64};

use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};
use crate::frontend::partial_eval::{partially_evaluate};
use crate::ir::explicate::{explicate_control};
use crate::backend::x64_backend::{ir_to_x64};
use crate::interpreter::{
    Interpreter, 
    interp_ast::AstInterpreter,
};

use crate::io::{get_line};

#[derive(PartialEq)]
enum ReplResult {
    BackToStart,
    KeepExecuting,
    Stop
}  

#[derive(Copy, Clone)]
struct ReplCommand {
    pub cmd: &'static str,
    pub help: &'static str,
    pub action: fn (&mut Repl) -> ReplResult,
}

pub struct Repl {
    buffer: String,
    current_line: String,
    commands: Vec<ReplCommand>,
    show_ast: bool,
    show_ir: bool,
    show_x64: bool,
    multiline_mode: bool,
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

                    ReplResult::BackToStart
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

                    ReplResult::BackToStart
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

                    ReplResult::BackToStart
                },
            },
            ReplCommand { cmd: ":grammer", help: "print the grammer", action: Repl::print_grammer },
            ReplCommand { cmd: ":quit", help: "exit the repl", action: Repl::quit },
            ReplCommand {
                cmd: ";;",
                help: "enter ;; to enter multiline mode and then ;; to evaluate",
                action: |r| {
                    if r.multiline_mode {
                        println!("--multiline mode off\n");

                        r.multiline_mode = false;

                        // edge case if user just toggles on then off without entering anything
                        if r.buffer == ";;" {
                            r.clear_input();
                            return ReplResult::BackToStart
                        }

                        r.current_line = r.buffer.clone();

                        ReplResult::KeepExecuting
                    } else {
                        println!("--multiline mode on\n");

                        r.multiline_mode = true;
                        ReplResult::BackToStart
                    }
                }
            },
        );

        Repl {
            buffer: "".to_owned(),
            current_line: "".to_owned(),
            commands: commands,
            show_ast: false,
            show_ir: false,
            show_x64: false,
            multiline_mode: false,
        }
    }

    fn print_grammer(&mut self) -> ReplResult {
        println!("
expr    ::= int | (read) | ('-' exp) | ('+' exp exp)
          | var | (let ([var exp]+) exp)
program ::= (exp)
        ");

        ReplResult::BackToStart
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

        ReplResult::BackToStart
    }

    fn handle_repl_command(&mut self, command: &str) -> ReplResult {
        let cmd = self.commands.iter().position(|&c| c.cmd == command);

        match cmd {
            Some(index) => {
                let repl_cmd = self.commands[index];
                let action = repl_cmd.action;
                action(self)
            },

            _ => ReplResult::BackToStart
        }
    }

    fn check_input_for_command(&mut self) -> ReplResult {
        let input = self.current_line.clone();

        if input == "" {
            ReplResult::BackToStart
        } else if input.starts_with(":") ||
                  input == ";;" {
                self.handle_repl_command(&input)
        } else if self.multiline_mode {
            ReplResult::BackToStart
        } else {
            ReplResult::KeepExecuting
        }
    }

    fn read_line(&mut self) {
        self.current_line = get_line();

        if self.multiline_mode {
            self.buffer.push_str(&self.current_line.clone());
        }
    }

    fn clear_input(&mut self) {
        self.current_line = "".to_owned();
        self.buffer = self.current_line.clone();
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        'repl_loop:loop {

            self.read_line();

            let what_to_do = self.check_input_for_command();

            match what_to_do {
                ReplResult::Stop => {
                    println!("Goodbye!");
                    break 'repl_loop;
                },

                ReplResult::BackToStart => {
                    continue 'repl_loop;
                },

                _ => {
                },
            }

            let mut l = Lexer::new(&self.current_line);

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

            let mut ast_interpreter = AstInterpreter::new(decomplified_program.clone());

            let mut interpreter = Interpreter::new(&mut ast_interpreter);

            let result = interpreter.run();

            if result.had_error {
                for error in result.errors {
                    println!("{}", error);
                }
                continue 'repl_loop;
            }

            let intermediate_repr = explicate_control(decomplified_program);

            if self.show_ir {
                println!("IR:");
                println!("{:#?}", intermediate_repr);
            }

            // doesn't matter which one
            println!("> {}\n", result.value.unwrap());

            let x64prog = ir_to_x64(intermediate_repr);
  
            if self.show_x64 {
                println!("{:#?}", x64prog);
            }

            self.buffer = "".to_owned();

        }

        Ok(())
    }
}