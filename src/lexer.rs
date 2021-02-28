use std::vec;

use crate::token;

use token::{Token, TokenType};

pub struct Lexer {
    source_code: Vec<char>,
    position: usize,
    end: usize,
    line: i32,
    column: i32,
}

trait VecChar {
    fn to_string(&self, start:usize, end:usize) -> String;
}

impl VecChar for Lexer {
    fn to_string(&self, start:usize, end:usize) -> String {
        self.source_code[start..end].into_iter().collect::<String>()
    }
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' ||
    c >= 'A' && c <= 'Z'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_id_start(c: char) -> bool {
    is_alpha(c)
}

fn is_id(c: char) -> bool {
    is_id_start(c) || is_digit(c) || c == '-'
}

impl Lexer {

    fn make_token(&mut self, t: TokenType, len: usize) -> token::Token {

        let start_pos = self.position;
        let end_pos = 
            if self.position + len >= self.end {
                self.end
            } else {
                self.position + len
            };

        let token = 
            token::Token {
                ttype: t,
                lexeme: self.to_string(start_pos, end_pos),
                col: self.column,
                line: self.line
            };

        self.advance_n(len);

        token
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.end
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source_code[self.position]
        }
    }

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.end {
            '\0'
        } else {
            self.source_code[self.position + 1]
        }
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
            self.peek()
        } else {
            '\0'
        }
    }

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    fn skip_whitespace(&mut self) {
        let mut c = self.peek();

        loop {
            match c {
                '\n' => { self.line += 1; self.column = 1;},
                ' ' => (),
                '\r' => (),
                '\t' => (),

                _ => break,
            }

            c = self.advance();
        }
    }

    fn number(&mut self) -> token::Token {

        let start = self.position;
        let col = self.column;
        let line = self.line;

        let mut c = self.advance();
        while is_digit(c) {
            c = self.advance();
        }

        token::Token {
            ttype: TokenType::Number,
            lexeme: self.to_string(start, self.position),
            col: col,
            line: line
        }
    }

    fn identifier(&mut self) -> token::Token {
        let start = self.position;
        let col = self.column;
        let line = self.line;

        let mut c = self.advance();
        while is_id(c) {
            c = self.advance();
        }

        token::Token {
            ttype: TokenType::Identifier,
            lexeme: self.to_string(start, self.position),
            col: col,
            line: line
        }
    }

    fn next_token(&mut self) -> token::Token {

        self.skip_whitespace();

        let token = {

            let c = self.peek();

            match c {
                '+'         => self.make_token(TokenType::Add, 1),
                '-'         => self.make_token(TokenType::Negate, 1),
                '('         => self.make_token(TokenType::Lparen, 1),
                ')'         => self.make_token(TokenType::Rparen, 1),
                '0'..='9'   => self.number(),
                '\0'        => self.make_token(TokenType::EndOfFile, 1),
                _           => {
                    if is_id_start(c) {
                        self.identifier()
                    } else {
                        self.make_token(TokenType::Error, 1)
                    }
                },
            }
        };

        token
    }

    pub fn lex(&mut self) -> Vec<Token> {

        let mut t = self.next_token();

        let mut token_vec: Vec<Token> = vec!();

        while t.ttype != TokenType::EndOfFile {
            token_vec.push(t);

            t = self.next_token();
        }

        return token_vec;
    }

    pub fn new(source: & str) -> Self {

        let src: Vec<char> = source.chars().collect();
        let len = src.len();

        Lexer {
            source_code: src,
            position: 0,
            end: len,
            line: 1,
            column: 1,
        }
    }
} 