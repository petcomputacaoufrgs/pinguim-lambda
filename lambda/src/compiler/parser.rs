#[cfg(test)]
mod test;

pub mod ast;
pub mod error;

use super::error::{Diagnostics, Error};
use crate::compiler::lexer::token::{Token, TokenType};
use ast::{Binding, Expr, Program, Symbol};
use error::{
    EmptyExpression, UnexpectedEndOfInput, UnexpectedToken,
    UnmatchedCloseParen, UnmatchedOpenParen,
};

/// Cria uma estrutura Parser e parsa a lista de tokens para um programa
///
/// - `tokens`: vetor de tokens
/// - `diagnostics`: vetor que armazena erros coletados durante a compilação
pub fn parse(
    tokens: Vec<Token>,
    diagnostics: &mut Diagnostics,
) -> Option<Program> {
    Parser::new(tokens).parse_program(diagnostics).ok().flatten()
}

trait ExprEnd {
    fn is_end(&self, token_type: Option<TokenType>) -> bool;
}

#[derive(Clone, Copy, Debug)]
struct BindingEnd;

impl ExprEnd for BindingEnd {
    fn is_end(&self, token_type: Option<TokenType>) -> bool {
        token_type == Some(TokenType::In)
            || token_type == Some(TokenType::Semicolon)
    }
}

#[derive(Clone, Copy, Debug)]
struct MainEnd;

impl ExprEnd for MainEnd {
    fn is_end(&self, token_type: Option<TokenType>) -> bool {
        token_type == None
    }
}

#[derive(Clone, Copy, Debug)]
struct ParenEnd;

impl ExprEnd for ParenEnd {
    fn is_end(&self, token_type: Option<TokenType>) -> bool {
        token_type == Some(TokenType::CloseParen)
    }
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
    fn require_current(
        &self,
        diagnostics: &mut Diagnostics,
    ) -> Result<&Token, Abort> {
        match self.current() {
            Some(token) => Ok(token),
            None => {
                diagnostics.raise(Error::with_no_span(UnexpectedEndOfInput));
                Err(Abort)
            }
        }
    }

    /// Incrementa o índice para o próximo token
    fn next(&mut self) {
        self.curr_token += 1;
    }

    fn previous(&self) -> Option<&Token> {
        self.curr_token
            .checked_sub(1)
            .map(|token_index| &self.tokens[token_index])
    }

    /// Confere se o próximo token é do tipo esperado, adicionando erro no diagnóstico quando não for
    ///
    /// - `expected_type`: tipo de token que é esperado encontrar
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn expect(
        &mut self,
        expected_type: TokenType,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == expected_type {
            self.next();
        } else {
            let expected_types = vec![expected_type];
            diagnostics.raise(Error::new(
                UnexpectedToken { expected_types },
                token.span,
            ));
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
        diagnostics: &mut Diagnostics,
    ) -> Result<bool, Abort> {
        let token = self.require_current(diagnostics)?;

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
    fn parse_program(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Program>, Abort> {
        let token = self.require_current(diagnostics)?;

        let bindings = if token.token_type == TokenType::Let {
            self.parse_let(diagnostics)?
        } else {
            Vec::new()
        };

        let main_expr_opt = self.parse_expression(diagnostics, &MainEnd)?;
        Ok(main_expr_opt
            .map(|main_expression| Program { main_expression, bindings }))
    }

    fn parse_let(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Vec<Binding>, Abort> {
        self.expect(TokenType::Let, diagnostics)?; // facilita isolamento dessa função (expect() X next())
        let mut bindings = Vec::new();

        while !self.check_expect(TokenType::In, diagnostics)? {
            if let Some(binding) = self.parse_binding(diagnostics)? {
                bindings.push(binding);
            }

            self.check_expect(TokenType::Semicolon, diagnostics)?;
        }

        Ok(bindings)
    }

    // ident = \x y . xyz
    fn parse_binding(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Binding>, Abort> {
        let name_opt = self.parse_binding_name(diagnostics)?;

        self.expect(TokenType::Equal, diagnostics)?;
        let expression_opt = self.parse_expression(diagnostics, &BindingEnd)?;

        Ok(name_opt
            .zip(expression_opt)
            .map(|(name, expression)| Binding { name, expression }))
    }

    fn parse_binding_name(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == TokenType::Identifier {
            Ok(Some(Symbol {
                content: token.content.clone(),
                span: token.span,
            }))
        } else {
            let expected_types = vec![TokenType::Identifier];
            diagnostics.raise(Error::new(
                UnexpectedToken { expected_types },
                token.span,
            ));

            Ok(None)
        }
    }

    fn parse_expression<E>(
        &mut self,
        diagnostics: &mut Diagnostics,
        expr_end: &E,
    ) -> Result<Option<Expr>, Abort>
    where
        E: ExprEnd + ?Sized,
    {
        let mut curr_expr: Option<Expr> = None;
        let mut is_empty = true;

        // a condição de parada vai ser algum tokentype tipo In ou Semicolon ou entao EOF
        while !expr_end.is_end(self.current().map(|token| token.token_type)) {
            let token = self.require_current(diagnostics)?;
            is_empty = false;

            match token.token_type {
                TokenType::Number => {
                    let number = Expr::Number(token.content.parse().unwrap());

                    self.stack_exprs(&mut curr_expr, number);
                }
                TokenType::Identifier => {
                    let ident = Expr::Variable(Symbol {
                        content: token.content.clone(),
                        span: token.span,
                    });

                    self.stack_exprs(&mut curr_expr, ident);
                }
                TokenType::Lambda => {
                    if let Some(lambda) =
                        self.parse_lambda(diagnostics, expr_end)?
                    {
                        self.stack_exprs(&mut curr_expr, lambda);
                    }
                }
                TokenType::OpenParen => {
                    let span = token.span;
                    self.next();

                    if let Some(sub_expr) =
                        self.parse_expression(diagnostics, &ParenEnd)?
                    {
                        self.stack_exprs(&mut curr_expr, sub_expr);
                        match self.current() {
                            Some(token)
                                if token.token_type
                                    == TokenType::CloseParen =>
                            {
                                self.next();
                            }
                            _ => {
                                diagnostics.raise(Error::new(
                                    UnmatchedOpenParen,
                                    span,
                                ));
                            }
                        }
                    }
                }
                TokenType::CloseParen => {
                    diagnostics
                        .raise(Error::new(UnmatchedCloseParen, token.span));
                }
                _ => {
                    let expected_types = vec![
                        TokenType::Number,
                        TokenType::Identifier,
                        TokenType::Lambda,
                        TokenType::OpenParen,
                    ];
                    diagnostics.raise(Error::new(
                        UnexpectedToken { expected_types },
                        token.span,
                    ));
                }
            }
        }

        if is_empty {
            let error = match self.previous() {
                Some(token) => Error::new(EmptyExpression, token.span),
                None => Error::with_no_span(EmptyExpression),
            };

            diagnostics.raise(error);
        }

        Ok(curr_expr)
    }

    fn stack_exprs(&self, curr_expr: &mut Option<Expr>, new_expr: Expr) {
        *curr_expr = match curr_expr.take() {
            Some(expr) => Some(Expr::Application {
                function: Box::new(expr),
                argument: Box::new(new_expr),
            }),
            None => Some(new_expr),
        }
    }

    fn parse_lambda<E>(
        &mut self,
        diagnostics: &mut Diagnostics,
        expr_end: &E,
    ) -> Result<Option<Expr>, Abort>
    where
        E: ExprEnd + ?Sized,
    {
        self.expect(TokenType::Lambda, diagnostics)?;

        let mut params = Vec::new();

        // até o ponto são os parâmetros da expressão lambda
        while !self.check_expect(TokenType::Dot, diagnostics)? {
            if let Some(param) = self.parse_param(diagnostics)? {
                params.push(param);
            }
        }

        // corpo da expressão lambda
        let lambda =
            self.parse_expression(diagnostics, expr_end)?.map(|lambda_body| {
                let mut expr = lambda_body;
                for param in params.into_iter().rev() {
                    expr =
                        Expr::Lambda { parameter: param, body: Box::new(expr) };
                }

                expr
            });

        Ok(lambda)
    }

    fn parse_param(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == TokenType::Identifier {
            Ok(Some(Symbol {
                content: token.content.clone(),
                span: token.span,
            }))
        } else {
            let expected_types = vec![TokenType::Identifier];
            diagnostics.raise(Error::new(
                UnexpectedToken { expected_types },
                token.span,
            ));

            Ok(None)
        }
    }
}
