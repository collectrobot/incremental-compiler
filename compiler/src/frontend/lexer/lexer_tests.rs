use crate::frontend::token::{Token, TokenType};
use crate::frontend::lexer::{Lexer};

#[test]
fn single_number() {

    let input = "2";

    let mut lexer = Lexer::new(input);

    let tokens = lexer.lex();

    let expected_tokens: Vec<Token> = vec!(
        Token { ttype: TokenType::Number, lexeme: "2".to_owned(), line: 1, col: 1 },
    );

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn constant_addition() {

    let input = "(+ 2 2)";

    let mut lexer = Lexer::new(input);

    let tokens = lexer.lex();

    let expected_tokens: Vec<Token> = vec!(
        Token { ttype: TokenType::Lparen, lexeme: "(".to_owned(), line: 1, col: 1 },
        Token { ttype: TokenType::Add, lexeme: "+".to_owned(), line: 1, col: 2 },
        Token { ttype: TokenType::Number, lexeme: "2".to_owned(), line: 1, col: 4 },
        Token { ttype: TokenType::Number, lexeme: "2".to_owned(), line: 1, col: 6 },
        Token { ttype: TokenType::Rparen, lexeme: ")".to_owned(), line: 1, col: 7 },
    );

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn let_statement() {

    let input = "(let ([x 10]) x)";

    let mut lexer = Lexer::new(input);

    let tokens = lexer.lex();

    let expected_tokens: Vec<Token> = vec!(
        Token { ttype: TokenType::Lparen, lexeme: "(".to_owned(), line: 1, col: 1 },
        Token { ttype: TokenType::Identifier, lexeme: "let".to_owned(), line: 1, col: 2 },
        Token { ttype: TokenType::Lparen, lexeme: "(".to_owned(), line: 1, col: 6 },
        Token { ttype: TokenType::Lbracket, lexeme: "[".to_owned(), line: 1, col: 7 },
        Token { ttype: TokenType::Identifier, lexeme: "x".to_owned(), line: 1, col: 8 },
        Token { ttype: TokenType::Number, lexeme: "10".to_owned(), line: 1, col: 10 },
        Token { ttype: TokenType::Rbracket, lexeme: "]".to_owned(), line: 1, col: 12 },
        Token { ttype: TokenType::Rparen, lexeme: ")".to_owned(), line: 1, col: 13 },
        Token { ttype: TokenType::Identifier, lexeme: "x".to_owned(), line: 1, col: 15 },
        Token { ttype: TokenType::Rparen, lexeme: ")".to_owned(), line: 1, col: 16 },
    );

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn error_unknown_caret() {

    let input = "^";

    let mut lexer = Lexer::new(input);

    let tokens = lexer.lex();

    let expected_tokens: Vec<Token> = vec!(
        Token { ttype: TokenType::Error, lexeme: "^".to_owned(), line: 1, col: 1 },
    );

    assert_eq!(tokens, expected_tokens);
}