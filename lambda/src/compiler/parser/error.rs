use crate::compiler::lexer::token::TokenType;
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct UnexpectedEndOfInput;

impl fmt::Display for UnexpectedEndOfInput {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Fim inesperado do código")
    }
}

impl Error for UnexpectedEndOfInput {}

#[derive(Debug, Clone)]
pub struct UnexpectedToken {
    pub expected_types: Vec<TokenType>,
}

impl fmt::Display for UnexpectedToken {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Token inesperado encontrado, esperava-se um ")?;

        let count = self.expected_types.len();

        if count > 1 {
            for expected_type in &self.expected_types[..count - 2] {
                write!(formatter, "\"{}\", ", expected_type)?;
            }

            write!(formatter, "\"{}\" ou ", self.expected_types[count - 2])?;
        }

        if count > 0 {
            write!(formatter, "\"{}\"", self.expected_types[count - 1])?;
        }

        Ok(())
    }
}

impl Error for UnexpectedToken {}

#[derive(Debug, Clone)]
pub struct EmptyLetBlockDeclared;

impl fmt::Display for EmptyLetBlockDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Bloco Let não pode ser declarado sem definições")
    }
}

impl Error for EmptyLetBlockDeclared {}

#[derive(Debug, Clone)]
pub struct UnmatchedOpenParen;

impl fmt::Display for UnmatchedOpenParen {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Parentesis aberto mas não fechado")
    }
}

impl Error for UnmatchedOpenParen {}

#[derive(Debug, Clone)]
pub struct UnmatchedCloseParen;

impl fmt::Display for UnmatchedCloseParen {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Parentesis fechado sobrando")
    }
}

impl Error for UnmatchedCloseParen {}
