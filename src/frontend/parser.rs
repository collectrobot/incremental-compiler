use super::token::{Token, TokenType};
use super::ast::{AstNode, Program};

//use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    parse_success: bool,
    errors: Vec<AstNode>,

    previous: Token,
}

impl Parser {

    fn rewind(&mut self, n: i64) -> Token {

        let rewind = n;
        let mut idx = (self.current as i64) - rewind;

        if idx < 0 {
            idx = 0;
        } else if idx > (self.current as i64) {
            idx = self.current as i64;
        }

        let token = self.tokens[(idx as usize)].clone();
        self.current = idx as usize;

        token
    }

    fn next(&mut self) -> Token {
        self.previous = self.peek(0);
        self.current += 1;

        let token = self.peek(0);

        token
    }

    fn peek(&self, n: i64) -> Token {
        let mut offset = (self.current as i64) + n;
        let max_len = (self.tokens.len() - 1) as i64;

        offset = 
            if offset > max_len {
                max_len
            } else if offset < 0 {
                0
            } else {
                offset
            };

        self.tokens[offset as usize].clone()
    }

    fn is(&self, ttype: TokenType) -> bool {
        self.current().ttype == ttype
    }

    fn next_is(&self, ttype: TokenType) -> bool {
        self.peek(1).ttype == ttype
    }

    fn expect(&mut self, ttype: TokenType) -> bool {
        if self.peek(0).ttype == ttype {
            self.next();
            true
        } else {
            false
        }
    }

    fn current(&self) -> Token {
        self.peek(0)
    } 

    pub fn parse_success(&self) -> bool {
        self.parse_success
    }

    pub fn print_errors(&self) {
        for error in &self.errors {
            println!("{:?}", error);
        }
    }

    fn error(&mut self) {
        self.parse_success = false;
    }

    fn make_error_node(&mut self, msg: String, offset: i64) -> AstNode {
        self.error();

        let error = 
            AstNode::Error {
                msg: msg,
                token: self.peek(offset)
            };

        self.errors.push(error.clone());

        error
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
            parse_success: true,
            errors: vec!(),
            previous: Token {
                ttype: TokenType::Error,
                lexeme: "".to_string(),
                line: -1,
                col: -1,
            },
        }
    }

    fn parse_number(&mut self) -> AstNode {
        let token = self.current();
        let node = AstNode::Int(token.lexeme.parse::<i64>().unwrap());

        self.next();

        node
    }

    fn parse_identifier(&mut self) -> AstNode {
        let token = self.current();
        self.next();

        match &token.lexeme[..] {
            "read" => {
                AstNode::Prim{ op: token.lexeme, args: vec!() }
            },

            // (let ([var exp]) (exp))
            "let" => {

                if !self.expect(TokenType::Lparen) {
                    return self.make_error_node("Expected a '('".to_owned(), 0)
                }

                if !self.expect(TokenType::Lbracket) {
                    return self.make_error_node("Expected a '['".to_owned(), 0)
                }

                let mut binding_vec: Vec<(String, AstNode)> = Vec::new();

                let mut keep_parsing = true;

                while keep_parsing {
                    // this should be the variable name e.g. "x"
                    let token = self.current();
                    let var = token.lexeme;

                    self.next();

                    let value = self.parse_expr(); // an expression that is bound to x

                    if self.next_is(TokenType::Lbracket) {
                        // we're at ']'
                        self.next(); // '['
                        self.next(); // begining of loop expects a variable
                    } else {
                        self.next(); // skip ']'
                        keep_parsing = false;
                    }

                    binding_vec.push((var, value));
                }

                if !self.expect(TokenType::Rparen) {
                    return self.make_error_node("Expected a ')'".to_owned(), 0)
                }


                let body = Box::new(self.parse_expr());

                AstNode::Let {
                    bindings: binding_vec,
                    body: body
                }
            },
            _ => {
                AstNode::Var {
                    name: token.lexeme
                }
            }
        }
    }

    fn parse_operator(&mut self) -> AstNode {
        let token = self.current();
        self.next();

        match token.ttype {
            TokenType::Add => {
                AstNode::Prim{
                    op: token.lexeme,
                    args: vec![self.parse_expr(), self.parse_expr()]
                }
            },
            TokenType::Negate => {
                AstNode::Prim{
                    op: token.lexeme,
                    args: vec![self.parse_expr()]
                }
            },

            _ => self.make_error_node("Unknown operator in parse_operator: ".to_owned(), -1)
        }
    }

    fn parse_expr(&mut self) -> AstNode {

        let token = self.current();

        match token.ttype {

            TokenType::Lparen => {
                self.next();

                let mut node = self.parse_expr();

                if !self.is(TokenType::Rparen) {
                    match node {
                        AstNode::Error {..} => { // don't there's already an error message
                        },
                        _ => {
                            node = self.make_error_node("Expected a closing ')', found: ".to_owned(), 0)
                        }
                    }
                }

                self.next();

                node
            },

            TokenType::Number => {
                self.parse_number()
            },

            TokenType::Add |
            TokenType::Negate => {
                self.parse_operator()
            },

            TokenType::Identifier => {

                self.parse_identifier()

            }
            _ => self.make_error_node("Unknown token in parse_expr: ".to_owned(), 0)
        }

    }

    fn parse_program(&mut self) -> Program {
        Program {
            info: (),
            exp: {
                if !self.is(TokenType::Lparen) {
                    self.make_error_node("Expected a '(', found: ".to_owned(), 0)
                } else {

                    self.next();
                    let mut node = self.parse_expr();
                    if !self.is(TokenType::Rparen) {
                        match node {
                            AstNode::Error {..} => { // don't there's already an error message
                            },
                            _ => {
                                node = self.make_error_node("Expected a ')', found: ".to_owned(), 0)
                            }
                        }
                    }

                    node
                }
            }
        }
    }

    pub fn parse(&mut self) -> Program {
        self.parse_program()
    }
}
