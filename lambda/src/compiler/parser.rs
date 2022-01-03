pub mod ast;

use crate::compiler::lexer::token::{Token, TokenType};
use ast::{Value, Program};
use indexmap::IndexMap;

pub fn parse(
    tokens: Vec<Token>
) -> Option<Value> {
    todo!()
}

struct Abort;

struct Parser {
    tokens: Vec<Token>,
    curr_token: usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            curr_token: 0,
        }
    }
    
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.curr_token)
    }

    fn require_current(&self) -> Result<&Token, Abort> {
        match self.current() {
            Some(&token) => Ok(&token),
            None => Err(Abort)
        }
    }

    fn  next(&mut self) {
        self.curr_token += 1;
    }

    fn expect(&mut self, expected_type: TokenType) -> Result<(), Abort> {
        let token = self.require_current()?;

        if token.token_type == expected_type {
            self.next(); 
        } else {
            let expected_types = vec![expected_type];
            //Acrescentar Diagnostics
        }

        Ok(())
    }

    fn check_expect(&mut self, expected_type: TokenType) -> Result<bool, Abort> {
        let token = self.require_current()?;

        if token.token_type == expected_type {
            self.next();
            Ok(true) 
        } else {
            Ok(false)
        }
    }

    fn parse_program(&mut self,) -> Result<Option<Program>, Abort> {
        let mut program: ast::Value;
        let mut let_option: IndexMap<String, Value>;
        let mut let_declared = false;
        let mut in_declared = false;

        while let Some(token) = self.current() {
            let token_span = token.span;

            match token.token_type {
                TokenType::Let => {
                    if !let_declared {
                        let_option = self.parse_let()?;
                        let_declared = true;
                    }
                },
                TokenType::Lambda => {

                },
                _ => //errouuu
            }
        }

        todo!()
    }

    fn parse_let(&mut self) -> Result<IndexMap<String, Value>, Abort> {

        // Basicamente vai passar por cada expressão (separador de expressões: ponto e vírgula) e chamar outro método que parsa 1 expressão por vez
        todo!()
    }

    fn parse_expr(&mut self) -> Option<Value> {
        todo!()
    }

    fn parse_expr_name(&mut self) -> Result<Option<String>, Abort> {
        let token = self.require_current()?;

        if token.token_type == TokenType::Identifier {
            let expr_name = token.content.clone();

            self.next();
            Ok(Some(expr_name))
        } else {
            let expected_types = vec![TokenType::Identifier];
            // diagnostics.raise(Error::new(
            //      UnexpectedToken { expected_types },
            //      token.span,
            // ))
            todo!()
        }
    }

    fn parse_func_expr(&mut self) -> Result<Value, Abort> {
        let mut expr_value: Value;

        self.expect(TokenType::Equal);

        todo!()
    }
}