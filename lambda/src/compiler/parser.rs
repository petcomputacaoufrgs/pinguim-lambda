pub mod ast;

use super::error::{Diagnostics, Error};
use crate::compiler::lexer::token::{Token, TokenType};
use ast::{Binding, Expr, Program, Symbol};
use indexmap::IndexMap;

/// Cria uma estrutura Parser e parsa a lista de tokens para um programa
///
/// - `tokens`: vetor de tokens
/// - `diagnostics`: vetor que armazena erros coletados durante a compilação
pub fn parse(
    tokens: Vec<Token>,
    diagnostics: &mut Diagnostics,
) -> Option<Program> {
    todo!()
}

#[derive(Debug)]
/// Estrutura responsável por parar o parser em situações críticas
struct Abort;

struct Parser {
    ///
    /// - `tokens`: vetor de tokens a serem parsados
    tokens: Vec<Token>,
    ///
    /// - `curr_token`: índice do token que está sendo parsado
    curr_token: usize,
}

impl Parser {
    /// Cria uma nova estrutura de Parser
    ///
    /// - `tokens`: vetor de tokens
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, curr_token: 0 }
    }
    /// Pega o token o qual está sendo parsado no momento dado seu índice `curr_token`
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.curr_token)
    }

    /// Pega o token o qual está sendo parsado no momento e garante que ele exista
    ///
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn require_current(&self) -> Result<&Token, Abort> {
        match self.current() {
            Some(token) => Ok(token),
            None => Err(Abort),
        }
    }

    /// Incrementa o índice para o próximo token
    fn next(&mut self) {
        self.curr_token += 1;
    }

    /// Confere se o próximo token é do tipo esperado, adicionando erro no diagnóstico quando não for
    ///
    /// - `expected_type`: tipo de token que é esperado encontrar
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
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

    /// Confere se o próximo token é do tipo esperado, retornando true se for, false se não for
    ///
    /// - `expected_type`: tipo de token esperado
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn check_expect(
        &mut self,
        expected_type: TokenType,
    ) -> Result<bool, Abort> {
        let token = self.require_current()?;

        if token.token_type == expected_type {
            self.next();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Faz o parse do vetor de tokens em um programa
    ///
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_program(&mut self) -> Result<Option<Program>, Abort> {
        let token = self.require_current()?;

        let bindings = if token.token_type == TokenType::Let {
            self.parse_let()?
        } else {
            IndexMap::new()
        };

        let main_expression = self.parse_expression()?;
        Ok(Some(Program { main_expression, bindings }))
    }

    fn parse_let(&mut self) -> Result<IndexMap<String, Binding>, Abort> {
        self.expect(TokenType::Let)?; // facilita isolamento dessa função (expect() X next())
        let mut bindings = IndexMap::<String, Binding>::new();

        while !self.check_expect(TokenType::In)? {
            if let Some(binding) = self.parse_binding()? {
                bindings.insert(binding.name.content.clone(), binding);
            }

            self.check_expect(TokenType::Semicolon)?;
        }

        Ok(bindings)
    }

    // ident = \x y . xyz
    fn parse_binding(&mut self) -> Result<Option<Binding>, Abort> {
        let name = self.parse_binding_name();

        self.expect(TokenType::Equal);
        self.next();

        todo!()
    }

    fn parse_binding_name(&mut self) -> Result<Option<String>, Abort> {
        let token = self.require_current()?;

        if token.token_type == TokenType::Identifier {
            let expr_name = token.content.clone();

            Ok(Some(expr_name))
        } else {
            let expected_types = vec![TokenType::Identifier];
            //error: UnexpectedToken
            todo!()
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, Abort> {
        let mut expr_value: Expr;

        self.next();

        todo!()
    }
}
