#[cfg(test)]
mod test;

pub mod error;
pub mod token;

use super::error::{Diagnostics, Error};
use error::{BadCommentStart, InvalidChar};
use crate::compiler::position::Span;
use std::{error::Error as StdError, iter::Peekable, str};
use token::{Token, TokenType};

pub fn generate_tokens(
    source: &str,
    diagnostics: &mut Diagnostics,
) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(source);

    while let Some(token) = lexer.generate_token(diagnostics) {
        tokens.push(token);
    }
    
    tokens
}

#[derive(Debug, Clone)]
enum Failure {
    EndOfInput,
    TryAgain,
}

#[derive(Debug, Clone)]
struct Lexer<'src> {
    source: Peekable<str::Chars<'src>>,
    token_content: String,
    token_span: Span,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source: source.chars().peekable(),
            token_span: Span::default(),
            token_content: String::new(),
        }
    }

    fn generate_token(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Option<Token> {
        loop {
            match self.try_generate_token(diagnostics) {
                Ok(token) => break Some(token),
                Err(Failure::EndOfInput) => break None,
                Err(Failure::TryAgain) => (),
            }
        }
    }

    fn try_generate_token (
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Token, Failure> {
        self.skip_discardable(diagnostics);

        self.clear_current();

        if self.is_identifier() {
            Ok(self.tokenize_ident())
        } else if let Some(typ) = self.match_punctuation() {
            Ok(self.tokenize_punct(typ))
        } else {
            match self.source.peek() {
                Some(&character) => {
                    self.next_char();
                    self.raise(diagnostics, InvalidChar { character });
                    Err(Failure::TryAgain)
                },
                None => Err(Failure::EndOfInput),
            }
        }
    }

    fn next_char(&mut self) {
        if let Some(character) = self.source.next() {
            self.token_span.update(character);
            self.token_content.push(character);
        }
    }

    fn clear_current(&mut self) {
        self.token_content.clear();
        self.token_span.finish();
    }

    fn skip_discardable(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) {
        while self.skip_whitespace() || self.skip_comment(diagnostics) {}
    } 

    fn skip_whitespace(&mut self) -> bool {
        let mut skipped = false;
        while self.is_whitespace() {
            self.next_char();
            skipped = true;
        }
        skipped
    }

    fn skip_comment(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> bool {
        if self.is_comment_start() {
            self.clear_current();
            self.next_char();
            if self.is_comment_start() {
                self.next_char();
            } else {
                self.raise(diagnostics, BadCommentStart)
            }
            while !self.is_comment_end() {
                self.next_char();
            }
            true
        } else {
            false
        }
    }

    fn tokenize_ident(&mut self) -> Token {
        let mut only_number = true;
        while self.is_identifier() {
            only_number = only_number && self.is_number();
            self.next_char();
        }
        
        let token_type = if only_number {
            TokenType::Number
        } else if let Some(keyword) = self.match_keyword() {
            keyword
        } else {
            TokenType::Identifier
        };
        
        self.make_token(token_type)
    }

    fn tokenize_punct(&mut self, token_type: TokenType) -> Token {
        self.next_char();
        self.make_token(token_type)
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            content: self.token_content.clone(),
            span: self.token_span,
        }
    }

    fn is_whitespace(&mut self) -> bool {
        match self.source.peek() {
            Some(&character) => character.is_whitespace(),
            None => false,
        }
    }

    fn is_comment_start(&mut self) -> bool {
        match self.source.peek() {
            Some(&character) => character == '-',
            None => false,
        }
    }

    fn is_comment_end(&mut self) -> bool {
        match self.source.peek() {
            Some(&character) => character == '\n',
            None => true,
        }
    }

    fn is_identifier(&mut self) -> bool {
        match self.source.peek() {
            Some('_') => true,
            Some(&character) => character.is_ascii_alphanumeric(),
            None => false,
        }
    }

    fn match_keyword(&self) ->  Option<TokenType> {
        match self.token_content.as_str() {
            "let" => Some(TokenType::Let),
            "in" => Some(TokenType::In),
            _ => None,
        }
    }

    fn match_punctuation(&mut self) -> Option<TokenType> {
        match self.source.peek() {
            Some(&character) => match character {
                '=' => Some(TokenType::Equal),
                '.' => Some(TokenType::Dot),
                '(' => Some(TokenType::OpenParen),
                ')' => Some(TokenType::CloseParen),
                '\\' => Some(TokenType::Lambda),
                _ => None,
            }
            _ => None,
        }
    }

}