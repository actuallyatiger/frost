#![allow(dead_code)]

use std::char;

use tokens::Token;

pub mod span;
pub mod tokens;

struct Lexer {
    pub input: String,
    pub position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    /// Peek at a character in the input without advancing the position.
    /// Takes an `offset` from the current position.
    /// Returns `None` if the end of the input is reached.
    fn peek(&self, offset: usize) -> Option<char> {
        self.input.chars().nth(self.position + offset)
    }

    /// Advance the position in the input by `count` characters.
    fn advance(&mut self, count: usize) {
        assert!(count > 0, "Count must be greater than zero");
        self.position += count;
        // Ensure the position does not exceed the length of the input
        if self.position > self.input.len() {
            self.position = self.input.len();
        }
    }

    /// Get the current character in the input.
    /// Returns `None` if the end of the input is reached.
    fn next(&mut self) -> Option<char> {
        let ch = self.peek(0);
        self.advance(1);
        ch
    }

    /// Consumes `count` characters and returns the specified TokenKind.
    fn consume(&mut self, kind: tokens::TokenKind, count: usize) -> Token {
        assert!(count > 0, "Count must be greater than zero");
        assert!(
            self.position + count <= self.input.len(),
            "Count exceeds input length"
        );
        let start = self.position;
        self.advance(count);
        Token::new(kind, start, count)
    }

    /// Reads a keyword from the input.
    fn read_keyword(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.peek(0) {
            if matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                self.advance(1);
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn read_integer(&mut self) -> isize {
        let start = self.position;
        while let Some(char) = self.peek(0) {
            if char.is_ascii_digit() {
                self.advance(1);
            } else {
                break;
            }
        }
        self.input[start..self.position].parse().unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        use tokens::TokenKind::*;

        let Some(ch) = self.peek(0) else {
            return Token::new(tokens::TokenKind::Eof, self.position, 0);
        };
        match ch {
            ch if ch.is_whitespace() => {
                self.advance(1);
                self.next_token()
            }
            '(' => self.consume(LParen, 1),
            ')' => self.consume(RParen, 1),
            '[' => self.consume(LBracket, 1),
            ']' => self.consume(RBracket, 1),
            '{' => self.consume(LBrace, 1),
            '}' => self.consume(RBrace, 1),
            ',' => self.consume(Comma, 1),
            ':' => self.consume(Colon, 1),
            '=' => {
                if self.peek(1) == Some('=') {
                    self.consume(Equals, 2)
                } else {
                    self.consume(Assign, 1)
                }
            }
            '+' => self.consume(Plus, 1),
            '-' => self.consume(Minus, 1),
            '*' => self.consume(Multiply, 1),
            '/' => {
                if self.peek(1) == Some('/') {
                    // Consume the comment
                    while self.peek(0) != Some('\n') && self.peek(0).is_some() {
                        self.advance(1);
                    }
                    self.next_token()
                } else {
                    self.consume(Divide, 1)
                }
            }
            '%' => self.consume(Modulus, 1),
            '<' => {
                if self.peek(1) == Some('=') {
                    self.consume(LessThanOrEqual, 2)
                } else {
                    self.consume(LessThan, 1)
                }
            }
            '>' => {
                if self.peek(1) == Some('=') {
                    self.consume(GreaterThanOrEqual, 2)
                } else {
                    self.consume(GreaterThan, 1)
                }
            }
            '&' => {
                if self.peek(1) == Some('&') {
                    self.consume(And, 2)
                } else {
                    panic!(
                        "Unexpected character: '{}' at position: {}",
                        ch, self.position
                    );
                }
            }
            '|' => {
                if self.peek(1) == Some('|') {
                    self.consume(Or, 2)
                } else {
                    panic!(
                        "Unexpected character: '{}' at position: {}",
                        ch, self.position
                    );
                }
            }
            '!' => {
                if self.peek(1) == Some('=') {
                    self.consume(NotEquals, 2)
                } else {
                    self.consume(Not, 1)
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let start = self.position;
                let kw = self.read_keyword();
                match kw.as_str() {
                    "val" => Token::new(Val, start, 3),
                    "var" => Token::new(Var, start, 3),
                    "fn" => Token::new(Fn, start, 2),
                    _ => Token::new(Identifier(kw.clone()), start, kw.len()),
                }
            }
            '0'..='9' => {
                let start = self.position;
                let value = self.read_integer();
                Token::new(IntLiteral(value), start, self.position - start)
            }

            ch => {
                panic!(
                    "Unexpected character: '{}' at position: {}",
                    ch, self.position
                );
            }
        }
    }
}

#[cfg(test)]
mod lexer {
    use tokens::TokenKind;

    use super::*;

    fn expect_token(lexer: &mut Lexer, expected: TokenKind) {
        let token = lexer.next_token();
        assert_eq!(
            token.kind, expected,
            "Expected token: {:?}, but got: {:?}",
            expected, token.kind
        );
    }

    #[test]
    fn parse_val() {
        use tokens::TokenKind::*;

        let input = "val x = 5";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Val);
        expect_token(&mut lexer, Identifier("x".to_string()));
        expect_token(&mut lexer, Assign);
        expect_token(&mut lexer, IntLiteral(5));
        expect_token(&mut lexer, Eof);

        // Check that the lexer has reached the end of the input
        assert_eq!(lexer.position, input.len());
    }

    #[test]
    fn parse_multiline() {
        use tokens::TokenKind::*;

        let input = "val x = 5\nval y = 10";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Val);
        expect_token(&mut lexer, Identifier("x".to_string()));
        expect_token(&mut lexer, Assign);
        expect_token(&mut lexer, IntLiteral(5));
        expect_token(&mut lexer, Val);
        expect_token(&mut lexer, Identifier("y".to_string()));
        expect_token(&mut lexer, Assign);
        expect_token(&mut lexer, IntLiteral(10));
        expect_token(&mut lexer, Eof);

        // Check that the lexer has reached the end of the input
        assert_eq!(lexer.position, input.len());
    }

    #[test]
    fn parse_arithmetic_operators() {
        use tokens::TokenKind::*;

        let input = "1 + 2 - 3 * 4 / 5 % 6";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, IntLiteral(1));
        expect_token(&mut lexer, Plus);
        expect_token(&mut lexer, IntLiteral(2));
        expect_token(&mut lexer, Minus);
        expect_token(&mut lexer, IntLiteral(3));
        expect_token(&mut lexer, Multiply);
        expect_token(&mut lexer, IntLiteral(4));
        expect_token(&mut lexer, Divide);
        expect_token(&mut lexer, IntLiteral(5));
        expect_token(&mut lexer, Modulus);
        expect_token(&mut lexer, IntLiteral(6));
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_comparison_operators() {
        use tokens::TokenKind::*;

        let input = "a < b > c <= d >= e == f != g";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Identifier("a".to_string()));
        expect_token(&mut lexer, LessThan);
        expect_token(&mut lexer, Identifier("b".to_string()));
        expect_token(&mut lexer, GreaterThan);
        expect_token(&mut lexer, Identifier("c".to_string()));
        expect_token(&mut lexer, LessThanOrEqual);
        expect_token(&mut lexer, Identifier("d".to_string()));
        expect_token(&mut lexer, GreaterThanOrEqual);
        expect_token(&mut lexer, Identifier("e".to_string()));
        expect_token(&mut lexer, Equals);
        expect_token(&mut lexer, Identifier("f".to_string()));
        expect_token(&mut lexer, NotEquals);
        expect_token(&mut lexer, Identifier("g".to_string()));
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_logical_operators() {
        use tokens::TokenKind::*;

        let input = "a && b || !c";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Identifier("a".to_string()));
        expect_token(&mut lexer, And);
        expect_token(&mut lexer, Identifier("b".to_string()));
        expect_token(&mut lexer, Or);
        expect_token(&mut lexer, Not);
        expect_token(&mut lexer, Identifier("c".to_string()));
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_brackets_and_delimiters() {
        use tokens::TokenKind::*;

        let input = "( ) [ ] { } , :";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, LParen);
        expect_token(&mut lexer, RParen);
        expect_token(&mut lexer, LBracket);
        expect_token(&mut lexer, RBracket);
        expect_token(&mut lexer, LBrace);
        expect_token(&mut lexer, RBrace);
        expect_token(&mut lexer, Comma);
        expect_token(&mut lexer, Colon);
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_keywords() {
        use tokens::TokenKind::*;

        let input = "val var fn";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Val);
        expect_token(&mut lexer, Var);
        expect_token(&mut lexer, Fn);
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_identifiers() {
        use tokens::TokenKind::*;

        let input = "abc x123 camelCase snake_case aB_1c_";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Identifier("abc".to_string()));
        expect_token(&mut lexer, Identifier("x123".to_string()));
        expect_token(&mut lexer, Identifier("camelCase".to_string()));
        expect_token(&mut lexer, Identifier("snake_case".to_string()));
        expect_token(&mut lexer, Identifier("aB_1c_".to_string()));
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_function_declaration() {
        use tokens::TokenKind::*;

        let input = "fn add(a: int, b: int): int { a + b }";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Fn);
        expect_token(&mut lexer, Identifier("add".to_string()));
        expect_token(&mut lexer, LParen);
        expect_token(&mut lexer, Identifier("a".to_string()));
        expect_token(&mut lexer, Colon);
        expect_token(&mut lexer, Identifier("int".to_string()));
        expect_token(&mut lexer, Comma);
        expect_token(&mut lexer, Identifier("b".to_string()));
        expect_token(&mut lexer, Colon);
        expect_token(&mut lexer, Identifier("int".to_string()));
        expect_token(&mut lexer, RParen);
        expect_token(&mut lexer, Colon);
        expect_token(&mut lexer, Identifier("int".to_string()));
        expect_token(&mut lexer, LBrace);
        expect_token(&mut lexer, Identifier("a".to_string()));
        expect_token(&mut lexer, Plus);
        expect_token(&mut lexer, Identifier("b".to_string()));
        expect_token(&mut lexer, RBrace);
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_complex_expression() {
        use tokens::TokenKind::*;

        let input = "val result = (a + b) * (c - d) / (e % f)";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Val);
        expect_token(&mut lexer, Identifier("result".to_string()));
        expect_token(&mut lexer, Assign);
        expect_token(&mut lexer, LParen);
        expect_token(&mut lexer, Identifier("a".to_string()));
        expect_token(&mut lexer, Plus);
        expect_token(&mut lexer, Identifier("b".to_string()));
        expect_token(&mut lexer, RParen);
        expect_token(&mut lexer, Multiply);
        expect_token(&mut lexer, LParen);
        expect_token(&mut lexer, Identifier("c".to_string()));
        expect_token(&mut lexer, Minus);
        expect_token(&mut lexer, Identifier("d".to_string()));
        expect_token(&mut lexer, RParen);
        expect_token(&mut lexer, Divide);
        expect_token(&mut lexer, LParen);
        expect_token(&mut lexer, Identifier("e".to_string()));
        expect_token(&mut lexer, Modulus);
        expect_token(&mut lexer, Identifier("f".to_string()));
        expect_token(&mut lexer, RParen);
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_comments() {
        use tokens::TokenKind::*;

        let input = "val x = 5 // This is a comment\nval y = 10";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Val);
        expect_token(&mut lexer, Identifier("x".to_string()));
        expect_token(&mut lexer, Assign);
        expect_token(&mut lexer, IntLiteral(5));
        // Comment should be skipped
        expect_token(&mut lexer, Val);
        expect_token(&mut lexer, Identifier("y".to_string()));
        expect_token(&mut lexer, Assign);
        expect_token(&mut lexer, IntLiteral(10));
        expect_token(&mut lexer, Eof);
    }

    #[test]
    fn parse_whitespace() {
        use tokens::TokenKind::*;

        let input = "  val  x  =  5  ";
        let mut lexer = Lexer::new(input.to_string());

        expect_token(&mut lexer, Val);
        expect_token(&mut lexer, Identifier("x".to_string()));
        expect_token(&mut lexer, Assign);
        expect_token(&mut lexer, IntLiteral(5));
        expect_token(&mut lexer, Eof);

        // Even with extra whitespace, the lexer should correctly reach the end
        assert_eq!(lexer.position, input.len());
    }
}
