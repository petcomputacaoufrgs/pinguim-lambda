pub mod token;

use crate::compiler::position::Span;
use std::{error::Error as StdError, iter::Peekable, str};
use token::{Token, TokenType};

pub fn generate_tokens(
    source: &str,
) -> Vec<Token> {
    todo!()
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
    ) -> Option<Token> {
        loop {
            match self.try_generate_token() {
                Ok(token) => break Some(token),
                Err(Failure::EndOfInput) => break None,
                Err(Failure::TryAgain) => (),
            }
        }
    }

    fn try_generate_token (
        &mut self,
    ) -> Result<Token, Failure> {
        self.skip_discardable();

        self.clear_current();

        todo!()
    }

    fn skip_discardable(&mut self) {
        while self.skip_whitespace() || self.skip_comment() {}
    } 

    fn skip_whitespace(&mut self) -> bool {
        let mut skipped = false;
        while self.is_whitespace() {
            self.next_char();
            skipped = true;
        }
        skipped
    }

    fn skip_comment(&mut self) -> bool {
        if self.is_comment_start() {
            self.clear_current();
            self.next_char();
            if self.is_comment_start() {
                self.next_char();
            } else {
                //self.raise(diagnostics, BadCommentStart)
            }
            while !self.is_comment_end() {
                self.next_char();
            }
            true
        } else {
            false
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

}