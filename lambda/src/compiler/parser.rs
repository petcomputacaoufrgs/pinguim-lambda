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
    Parser::new(tokens).parse_program().ok().flatten()
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

        let main_expr_opt =
            self.parse_expression(|token_type| token_type == None)?;
        Ok(main_expr_opt
            .map(|main_expression| Program { main_expression, bindings }))
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
        let name_opt = self.parse_binding_name()?;

        self.expect(TokenType::Equal)?;
        let expression_opt = self.parse_expression(|token_type| {
            token_type == Some(TokenType::In)
                || token_type == Some(TokenType::Semicolon)
        })?;

        Ok(name_opt
            .zip(expression_opt)
            .map(|(name, expression)| Binding { name, expression }))
    }

    fn parse_binding_name(&mut self) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current()?;

        if token.token_type == TokenType::Identifier {
            Ok(Some(Symbol {
                content: token.content.clone(),
                span: token.span,
            }))
        } else {
            let expected_types = vec![TokenType::Identifier];
            //error: UnexpectedToken
            todo!()
        }
    }

    fn parse_expression<F>(
        &mut self,
        mut is_end: F,
    ) -> Result<Option<Expr>, Abort>
    where
        F: FnMut(Option<TokenType>) -> bool,
    {
        let mut curr_expr: Option<Expr> = None;

        // a condição de parada vai ser algum tokentype tipo In ou Semicolon ou entao EOF
        while !is_end(self.current().map(|token| token.token_type)) {
            let token = self.require_current()?;

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
                    if let Some(lambda) = self.parse_lambda(&mut is_end)? {
                        self.stack_exprs(&mut curr_expr, lambda);
                    }
                }
                TokenType::OpenParen => {
                    self.next();

                    if let Some(sub_expr) =
                        self.parse_expression(|token_type| {
                            token_type == Some(TokenType::CloseParen)
                        })?
                    {
                        self.stack_exprs(&mut curr_expr, sub_expr);
                        self.next();
                    }
                }
                TokenType::CloseParen => {
                    // erro
                }
                _ => {
                    // erro
                }
            }
        }

        Ok(curr_expr) // REVISAR ERRO
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

    fn parse_lambda<F>(&mut self, is_end: F) -> Result<Option<Expr>, Abort>
    where
        F: FnMut(Option<TokenType>) -> bool,
    {
        self.expect(TokenType::Lambda)?;

        let mut params = Vec::new();

        // até o ponto são os parâmetros da expressão lambda
        while !self.check_expect(TokenType::Dot)? {
            if let Some(param) = self.parse_param()? {
                params.push(param);
            }
        }

        // corpo da expressão lambda
        let lambda = self.parse_expression(is_end)?.map(|lambda_body| {
            let mut expr = lambda_body;
            for param in params.into_iter().rev() {
                expr = Expr::Lambda { parameter: param, body: Box::new(expr) };
            }

            expr
        });

        Ok(lambda)
    }

    fn parse_param(&mut self) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current()?;

        if token.token_type == TokenType::Identifier {
            Ok(Some(Symbol {
                content: token.content.clone(),
                span: token.span,
            }))
        } else {
            let expected_types = vec![TokenType::Identifier];
            //error: IdentifierExpected/UnexpectedToken
            todo!()
        }
    }
}
