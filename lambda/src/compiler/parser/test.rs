use super::{ast, parse};
use crate::compiler::lexer::generate_tokens;
use crate::compiler::{
    error::Diagnostics,
    position::{Position, Span},
};

/*
Testes caso de sucesso
- capacidade de parsear identificadores
- capacidade de parsear lambdas
- capacidade de "empilhar" aplicações corretamente
- capacidade de balancear parentesis
- capacidade de parsear programas sem let
- capacidade de parsear programas com let (1 binding)
- capacidade de parsear programas com let (mais de 1 binding)
- se o ponto-e-vírgula final realmente é opcional
*/

#[test]
fn parse_identifier() {
    let source_code = "id";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Variable(ast::Symbol {
                content: String::from("id"),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        utf8_index: 0,
                        utf16_index: 0,
                    },
                    end: Position {
                        line: 1,
                        column: 3,
                        utf8_index: 2,
                        utf16_index: 2,
                    }
                }
            }),
            bindings: Vec::new(),
        })
    )
}
