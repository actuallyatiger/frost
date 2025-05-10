use crate::span::Span;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    Val, // val
    Var, // var
    Fn,  // fn

    // Control flow
    If,   // if
    Elif, // elif
    Else, // else

    // Syntax
    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]
    LBrace,   // {
    RBrace,   // }
    Comma,    // ,
    Colon,    // :
    Assign,   // =

    // Operators
    Plus,               // +
    Minus,              // -
    Multiply,           // *
    Exponent,           // ^
    Divide,             // /
    Modulus,            // %
    Equals,             // ==
    NotEquals,          // !=
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
    And,                // &&
    Or,                 // ||
    Not,                // !

    // Identifiers
    Identifier(String), // variable names, function names, etc.
    IntLiteral(isize),  // integer literals

    Eof, // End of file
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Span,
}

impl Token {
    /// Construct a new token with the given type, start position, and size
    pub fn new(kind: TokenKind, start: usize, size: usize) -> Self {
        Token {
            kind,
            pos: Span::new(start, start + size),
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenKind::Val => "val",
            TokenKind::Var => "var",
            TokenKind::Fn => "fn",
            TokenKind::If => "if",
            TokenKind::Elif => "elif",
            TokenKind::Else => "else",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Colon => ":",
            TokenKind::Assign => "=",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Multiply => "*",
            TokenKind::Exponent => "^",
            TokenKind::Divide => "/",
            TokenKind::Modulus => "%",
            TokenKind::Equals => "==",
            TokenKind::NotEquals => "!=",
            TokenKind::LessThan => "<",
            TokenKind::GreaterThan => ">",
            TokenKind::LessThanOrEqual => "<=",
            TokenKind::GreaterThanOrEqual => ">=",
            TokenKind::And => "&&",
            TokenKind::Or => "||",
            TokenKind::Not => "!",
            TokenKind::Identifier(name) => name,
            TokenKind::IntLiteral(value) => &value.to_string(),
            TokenKind::Eof => "EOF",
        };
        write!(f, "{}", str)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind, self.pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_display() {
        let token_type = TokenKind::Var;
        assert_eq!(token_type.to_string(), "var");
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenKind::Val, 0, 3);
        assert_eq!(token.to_string(), "val [0..3]");
    }
}
