use pinguim_language::position::Span;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum TokenType {
    Lambda,
    Identifier,
    Number,
    Equal,
    Let,
    In,
    Dot,
    OpenParen,
    CloseParen,
    Semicolon,
}

impl fmt::Display for TokenType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Lambda => {
                write!(formatter, "\\")
            }
            TokenType::Identifier => write!(formatter, "<identificador>"),
            TokenType::Number => write!(formatter, "<número>"),
            TokenType::Equal => write!(formatter, "="),
            TokenType::Let => write!(formatter, "let"),
            TokenType::In => write!(formatter, "in"),
            TokenType::Dot => write!(formatter, "."),
            TokenType::OpenParen => write!(formatter, "("),
            TokenType::CloseParen => write!(formatter, ")"),
            TokenType::Semicolon => write!(formatter, ";"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub span: Span,
}
