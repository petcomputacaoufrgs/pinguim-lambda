use crate::compiler::lexer::generate_tokens;
use crate::compiler::parser::{ast, parse};
use crate::compiler::{
    error::Diagnostics,
    position::{Position, Span},
};

/*
    - erro de código vazio OK!
    - erro de lambda sem parametro OK!
    - erro de lambda sem corpo OK!
        - até EOF OK!
        - até Semicolon  OK!
        - até In OK!
    - erro de faltando token equal no binding OK!
    - erro de parenteses aberto que não foi fechado
        - numa expressão delimitada por ";" ou "in"
        - numa expressão no final do código
    - erro de parenteses fechando sem parenteses aberto associado
    - erro de faltando "in" no código
    - erro de faltando "let" no início do código
    - erro de parênteses nos parâmetros do lambda
    - erro de lambda sem ponto
        - só vai parar no final do arquivo! (melhorar isso - talvez usar o ExprEnd)
*/

#[test]
fn empty_code() {
    let source_code = "";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Fim inesperado do código",]);
    assert_eq!(ast, None)
}

#[test]
fn lambda_without_params() {
    let source_code = "\\. f (f x)";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &["Lambda precisa ter pelo menos um parâmetro, na linha 1 e coluna 1",]
    );

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Application {
                function: Box::new(ast::Expr::Variable(ast::Symbol {
                    content: String::from("f"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 4,
                            utf8_index: 3,
                            utf16_index: 3,
                        },
                        end: Position {
                            line: 1,
                            column: 5,
                            utf8_index: 4,
                            utf16_index: 4,
                        }
                    }
                })),
                argument: Box::new(ast::Expr::Application {
                    function: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("f"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 7,
                                utf8_index: 6,
                                utf16_index: 6,
                            },
                            end: Position {
                                line: 1,
                                column: 8,
                                utf8_index: 7,
                                utf16_index: 7,
                            }
                        }
                    })),
                    argument: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("x"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 9,
                                utf8_index: 8,
                                utf16_index: 8,
                            },
                            end: Position {
                                line: 1,
                                column: 10,
                                utf8_index: 9,
                                utf16_index: 9,
                            }
                        }
                    }))
                })
            },
            bindings: Vec::new()
        })
    )
}

#[test]
fn lambda_without_body_until_eof() {
    let source_code = "\\f.";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Esperava-se uma expressão, na linha 1 e coluna 3",]);

    assert_eq!(ast, None)
}

#[test]
fn lambda_without_body_until_semicolon() {
    let source_code = "let\n    foo = \\f.;\nin\nbar";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Esperava-se uma expressão, na linha 2 e coluna 13",]);

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Variable(ast::Symbol {
                content: String::from("bar"),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 1,
                        utf8_index: 22,
                        utf16_index: 22,
                    },
                    end: Position {
                        line: 4,
                        column: 4,
                        utf8_index: 25,
                        utf16_index: 25,
                    }
                }
            }),
            bindings: Vec::new()
        })
    );
}

#[test]
fn lambda_without_body_until_in_keyword() {
    let source_code = "let\n    foo = \\f.\nin\nbar";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Esperava-se uma expressão, na linha 2 e coluna 13",]);

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Variable(ast::Symbol {
                content: String::from("bar"),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 1,
                        utf8_index: 21,
                        utf16_index: 21,
                    },
                    end: Position {
                        line: 4,
                        column: 4,
                        utf8_index: 24,
                        utf16_index: 24,
                    }
                }
            }),
            bindings: Vec::new()
        })
    );
}

#[test]
fn binding_missing_equal_symbol() {
    let source_code = "let\n    foo \\f x. x;\nin\nfoo";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Token inesperado encontrado, esperava-se um \"=\", na linha 2 e coluna 9",]);
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Variable(ast::Symbol {
                content: String::from("foo"),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 1,
                        utf8_index: 24,
                        utf16_index: 24,
                    },
                    end: Position {
                        line: 4,
                        column: 4,
                        utf8_index: 27,
                        utf16_index: 27,
                    }
                }
            }),
            bindings: vec![ast::Binding {
                name: ast::Symbol {
                    content: String::from("foo"),
                    span: Span {
                        start: Position {
                            line: 2,
                            column: 5,
                            utf8_index: 8,
                            utf16_index: 8,
                        },
                        end: Position {
                            line: 2,
                            column: 8,
                            utf8_index: 11,
                            utf16_index: 11,
                        }
                    }
                },
                expression: ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("f"),
                        span: Span {
                            start: Position {
                                line: 2,
                                column: 10,
                                utf8_index: 13,
                                utf16_index: 13,
                            },
                            end: Position {
                                line: 2,
                                column: 11,
                                utf8_index: 14,
                                utf16_index: 14,
                            }
                        }
                    },
                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("x"),
                            span: Span {
                                start: Position {
                                    line: 2,
                                    column: 12,
                                    utf8_index: 15,
                                    utf16_index: 15,
                                },
                                end: Position {
                                    line: 2,
                                    column: 13,
                                    utf8_index: 16,
                                    utf16_index: 16,
                                }
                            }
                        },
                        body: Box::new(ast::Expr::Variable(ast::Symbol {
                            content: String::from("x"),
                            span: Span {
                                start: Position {
                                    line: 2,
                                    column: 15,
                                    utf8_index: 18,
                                    utf16_index: 18,
                                },
                                end: Position {
                                    line: 2,
                                    column: 16,
                                    utf8_index: 19,
                                    utf16_index: 19,
                                }
                            }
                        }))
                    })
                }
            }]
        })
    );
}
