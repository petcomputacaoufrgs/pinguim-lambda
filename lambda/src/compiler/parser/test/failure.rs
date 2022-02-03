use crate::compiler::lexer::generate_tokens;
use crate::compiler::parser::{ast, parse};
use crate::compiler::{
    error::Diagnostics,
    position::{Position, Span},
};

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

#[test]
fn not_closed_open_paren_until_eof() {
    let source_code = "\\f.\\x.f (f (f x)";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &["Parentesis aberto mas não fechado, na linha 1 e coluna 9"]
    );

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Lambda {
                parameter: ast::Symbol {
                    content: String::from("f"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 2,
                            utf8_index: 1,
                            utf16_index: 1,
                        },
                        end: Position {
                            line: 1,
                            column: 3,
                            utf8_index: 2,
                            utf16_index: 2,
                        }
                    }
                },
                body: Box::new(ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("x"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 5,
                                utf8_index: 4,
                                utf16_index: 4,
                            },
                            end: Position {
                                line: 1,
                                column: 6,
                                utf8_index: 5,
                                utf16_index: 5,
                            }
                        }
                    },
                    body: Box::new(ast::Expr::Application {
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
                        argument: Box::new(ast::Expr::Application {
                            function: Box::new(ast::Expr::Variable(
                                ast::Symbol {
                                    content: String::from("f"),
                                    span: Span {
                                        start: Position {
                                            line: 1,
                                            column: 10,
                                            utf8_index: 9,
                                            utf16_index: 9,
                                        },
                                        end: Position {
                                            line: 1,
                                            column: 11,
                                            utf8_index: 10,
                                            utf16_index: 10,
                                        }
                                    }
                                }
                            )),
                            argument: Box::new(ast::Expr::Application {
                                function: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("f"),
                                        span: Span {
                                            start: Position {
                                                line: 1,
                                                column: 13,
                                                utf8_index: 12,
                                                utf16_index: 12,
                                            },
                                            end: Position {
                                                line: 1,
                                                column: 14,
                                                utf8_index: 13,
                                                utf16_index: 13,
                                            }
                                        }
                                    }
                                )),
                                argument: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("x"),
                                        span: Span {
                                            start: Position {
                                                line: 1,
                                                column: 15,
                                                utf8_index: 14,
                                                utf16_index: 14,
                                            },
                                            end: Position {
                                                line: 1,
                                                column: 16,
                                                utf8_index: 15,
                                                utf16_index: 15,
                                            }
                                        }
                                    }
                                ))
                            })
                        })
                    })
                })
            },
            bindings: Vec::new()
        })
    )
}

#[test]
fn not_closed_open_paren_until_semicolon() {
    let source_code = "let\n    three = \\f.\\x.f (f (f x);\nin\n   three";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &["Parentesis aberto mas não fechado, na linha 2 e coluna 21"]
    );

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Variable(ast::Symbol {
                content: String::from("three"),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 4,
                        utf8_index: 40,
                        utf16_index: 40,
                    },
                    end: Position {
                        line: 4,
                        column: 9,
                        utf8_index: 45,
                        utf16_index: 45,
                    },
                },
            }),
            bindings: vec![ast::Binding {
                name: ast::Symbol {
                    content: String::from("three"),
                    span: Span {
                        start: Position {
                            line: 2,
                            column: 5,
                            utf8_index: 8,
                            utf16_index: 8,
                        },
                        end: Position {
                            line: 2,
                            column: 10,
                            utf8_index: 13,
                            utf16_index: 13,
                        },
                    },
                },
                expression: ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("f"),
                        span: Span {
                            start: Position {
                                line: 2,
                                column: 14,
                                utf8_index: 17,
                                utf16_index: 17,
                            },
                            end: Position {
                                line: 2,
                                column: 15,
                                utf8_index: 18,
                                utf16_index: 18,
                            },
                        },
                    },
                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("x"),
                            span: Span {
                                start: Position {
                                    line: 2,
                                    column: 17,
                                    utf8_index: 20,
                                    utf16_index: 20,
                                },
                                end: Position {
                                    line: 2,
                                    column: 18,
                                    utf8_index: 21,
                                    utf16_index: 21,
                                },
                            },
                        },
                        body: Box::new(ast::Expr::Application {
                            function: Box::new(ast::Expr::Variable(
                                ast::Symbol {
                                    content: String::from("f"),
                                    span: Span {
                                        start: Position {
                                            line: 2,
                                            column: 19,
                                            utf8_index: 22,
                                            utf16_index: 22,
                                        },
                                        end: Position {
                                            line: 2,
                                            column: 20,
                                            utf8_index: 23,
                                            utf16_index: 23,
                                        },
                                    },
                                }
                            )),
                            argument: Box::new(ast::Expr::Application {
                                function: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("f"),
                                        span: Span {
                                            start: Position {
                                                line: 2,
                                                column: 22,
                                                utf8_index: 25,
                                                utf16_index: 25,
                                            },
                                            end: Position {
                                                line: 2,
                                                column: 23,
                                                utf8_index: 26,
                                                utf16_index: 26,
                                            },
                                        },
                                    },
                                )),
                                argument: Box::new(ast::Expr::Application {
                                    function: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("f"),
                                            span: Span {
                                                start: Position {
                                                    line: 2,
                                                    column: 25,
                                                    utf8_index: 28,
                                                    utf16_index: 28,
                                                },
                                                end: Position {
                                                    line: 2,
                                                    column: 26,
                                                    utf8_index: 29,
                                                    utf16_index: 29,
                                                },
                                            },
                                        },
                                    )),
                                    argument: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("x"),
                                            span: Span {
                                                start: Position {
                                                    line: 2,
                                                    column: 27,
                                                    utf8_index: 30,
                                                    utf16_index: 30,
                                                },
                                                end: Position {
                                                    line: 2,
                                                    column: 28,
                                                    utf8_index: 31,
                                                    utf16_index: 31,
                                                },
                                            },
                                        },
                                    )),
                                }),
                            }),
                        }),
                    }),
                },
            }],
        })
    )
}

#[test]
fn not_closed_open_paren_until_in_keyword() {
    let source_code = "let\n    three = \\f.\\x.f (f (f x)\nin\n   three";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &["Parentesis aberto mas não fechado, na linha 2 e coluna 21"]
    );

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Variable(ast::Symbol {
                content: String::from("three"),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 4,
                        utf8_index: 39,
                        utf16_index: 39,
                    },
                    end: Position {
                        line: 4,
                        column: 9,
                        utf8_index: 44,
                        utf16_index: 44,
                    },
                },
            }),
            bindings: vec![ast::Binding {
                name: ast::Symbol {
                    content: String::from("three"),
                    span: Span {
                        start: Position {
                            line: 2,
                            column: 5,
                            utf8_index: 8,
                            utf16_index: 8,
                        },
                        end: Position {
                            line: 2,
                            column: 10,
                            utf8_index: 13,
                            utf16_index: 13,
                        },
                    },
                },
                expression: ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("f"),
                        span: Span {
                            start: Position {
                                line: 2,
                                column: 14,
                                utf8_index: 17,
                                utf16_index: 17,
                            },
                            end: Position {
                                line: 2,
                                column: 15,
                                utf8_index: 18,
                                utf16_index: 18,
                            },
                        },
                    },
                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("x"),
                            span: Span {
                                start: Position {
                                    line: 2,
                                    column: 17,
                                    utf8_index: 20,
                                    utf16_index: 20,
                                },
                                end: Position {
                                    line: 2,
                                    column: 18,
                                    utf8_index: 21,
                                    utf16_index: 21,
                                },
                            },
                        },
                        body: Box::new(ast::Expr::Application {
                            function: Box::new(ast::Expr::Variable(
                                ast::Symbol {
                                    content: String::from("f"),
                                    span: Span {
                                        start: Position {
                                            line: 2,
                                            column: 19,
                                            utf8_index: 22,
                                            utf16_index: 22,
                                        },
                                        end: Position {
                                            line: 2,
                                            column: 20,
                                            utf8_index: 23,
                                            utf16_index: 23,
                                        },
                                    },
                                }
                            )),
                            argument: Box::new(ast::Expr::Application {
                                function: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("f"),
                                        span: Span {
                                            start: Position {
                                                line: 2,
                                                column: 22,
                                                utf8_index: 25,
                                                utf16_index: 25,
                                            },
                                            end: Position {
                                                line: 2,
                                                column: 23,
                                                utf8_index: 26,
                                                utf16_index: 26,
                                            },
                                        },
                                    },
                                )),
                                argument: Box::new(ast::Expr::Application {
                                    function: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("f"),
                                            span: Span {
                                                start: Position {
                                                    line: 2,
                                                    column: 25,
                                                    utf8_index: 28,
                                                    utf16_index: 28,
                                                },
                                                end: Position {
                                                    line: 2,
                                                    column: 26,
                                                    utf8_index: 29,
                                                    utf16_index: 29,
                                                },
                                            },
                                        },
                                    )),
                                    argument: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("x"),
                                            span: Span {
                                                start: Position {
                                                    line: 2,
                                                    column: 27,
                                                    utf8_index: 30,
                                                    utf16_index: 30,
                                                },
                                                end: Position {
                                                    line: 2,
                                                    column: 28,
                                                    utf8_index: 31,
                                                    utf16_index: 31,
                                                },
                                            },
                                        },
                                    )),
                                }),
                            }),
                        }),
                    }),
                },
            }],
        })
    );
}

#[test]
fn not_opened_close_paren() {
    let source_code = "(\\x. x) (\\x. x))";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &["Parentesis fechado sobrando, na linha 1 e coluna 16"]
    );

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Application {
                function: Box::new(ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("x"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 3,
                                utf8_index: 2,
                                utf16_index: 2,
                            },
                            end: Position {
                                line: 1,
                                column: 4,
                                utf8_index: 3,
                                utf16_index: 3,
                            }
                        }
                    },
                    body: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("x"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 6,
                                utf8_index: 5,
                                utf16_index: 5,
                            },
                            end: Position {
                                line: 1,
                                column: 7,
                                utf8_index: 6,
                                utf16_index: 6,
                            }
                        }
                    }))
                }),
                argument: Box::new(ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("x"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 11,
                                utf8_index: 10,
                                utf16_index: 10,
                            },
                            end: Position {
                                line: 1,
                                column: 12,
                                utf8_index: 11,
                                utf16_index: 11,
                            }
                        }
                    },
                    body: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("x"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 14,
                                utf8_index: 13,
                                utf16_index: 13,
                            },
                            end: Position {
                                line: 1,
                                column: 15,
                                utf8_index: 14,
                                utf16_index: 14,
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
fn missing_in_keyword() {
    let source_code = "let\n    id = \\x. x\n   i i";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Fim inesperado do código"]);

    assert_eq!(ast, None)
}

#[test]
fn missing_in_keyword_with_semicolon() {
    let source_code = "let\n    id = \\x. x;\n   i i";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors, 
        &[
            "Token inesperado encontrado, esperava-se um \"=\", na linha 3 e coluna 6", 
            "Fim inesperado do código"
        ]
    );

    assert_eq!(ast, None)
}

#[test]
fn missing_let_keyword_at_the_beginning() {
    let source_code = "    id = \\x. x\nin\n   id id";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors, 
        &[
            "Token inesperado encontrado, esperava-se um \"<número>\", \"<identificador>\", \"\\\" ou \"(\", na linha 1 e coluna 8",
            "Token inesperado encontrado, esperava-se um \"<número>\", \"<identificador>\", \"\\\" ou \"(\", de linha 2 e coluna 1, até coluna 2"
        ]
    );

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Application {
                function: Box::new(ast::Expr::Variable(ast::Symbol {
                    content: String::from("id"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 5,
                            utf8_index: 4,
                            utf16_index: 4,
                        },
                        end: Position {
                            line: 1,
                            column: 7,
                            utf8_index: 6,
                            utf16_index: 6,
                        }
                    }
                })),
                argument: Box::new(ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("x"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 11,
                                utf8_index: 10,
                                utf16_index: 10,
                            },
                            end: Position {
                                line: 1,
                                column: 12,
                                utf8_index: 11,
                                utf16_index: 11,
                            }
                        }
                    },
                    body: Box::new(ast::Expr::Application {
                        function: Box::new(ast::Expr::Application {
                            function: Box::new(ast::Expr::Variable(ast::Symbol {
                                content: String::from("x"),
                                span: Span {
                                    start: Position {
                                        line: 1,
                                        column: 14,
                                        utf8_index: 13,
                                        utf16_index: 13,
                                    },
                                    end: Position {
                                        line: 1,
                                        column: 15,
                                        utf8_index: 14,
                                        utf16_index: 14,
                                    }
                                }
                            })),
                            argument: Box::new(ast::Expr::Variable(ast::Symbol {
                                content: String::from("id"),
                                span: Span {
                                    start: Position {
                                        line: 3,
                                        column: 4,
                                        utf8_index: 21,
                                        utf16_index: 21,
                                    },
                                    end: Position {
                                        line: 3,
                                        column: 6,
                                        utf8_index: 23,
                                        utf16_index: 23,
                                    }
                                }
                            }))
                        }),
                        argument: Box::new(ast::Expr::Variable(ast::Symbol {
                            content: String::from("id"),
                            span: Span {
                                start: Position {
                                    line: 3,
                                    column: 7,
                                    utf8_index: 24,
                                    utf16_index: 24,
                                },
                                end: Position {
                                    line: 3,
                                    column: 9,
                                    utf8_index: 26,
                                    utf16_index: 26,
                                }
                            }
                        }))
                    })
                }),
            },
            bindings: Vec::new()
        })
    )
}

#[test]
fn applying_application_to_lambda_params() {
    let source_code = "\\x (y z). w";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors, 
        &[
            "Token inesperado encontrado, esperava-se um \"<identificador>\", na linha 1 e coluna 4",
            "Token inesperado encontrado, esperava-se um \"<identificador>\", na linha 1 e coluna 8"
        ]
    );

    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Lambda {
                parameter: ast::Symbol {
                    content: String::from("x"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 2,
                            utf8_index: 1,
                            utf16_index: 1,
                        },
                        end: Position {
                            line: 1,
                            column: 3,
                            utf8_index: 2,
                            utf16_index: 2,
                        }
                    }
                },
                body: Box::new(ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("y"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 5,
                                utf8_index: 4,
                                utf16_index: 4,
                            },
                            end: Position {
                                line: 1,
                                column: 6,
                                utf8_index: 5,
                                utf16_index: 5,
                            }
                        }
                    },
                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("z"),
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
                        },
                        body: Box::new(ast::Expr::Variable(ast::Symbol {
                            content: String::from("w"),
                            span: Span {
                                start: Position {
                                    line: 1,
                                    column: 11,
                                    utf8_index: 10,
                                    utf16_index: 10,
                                },
                                end: Position {
                                    line: 1,
                                    column: 12,
                                    utf8_index: 11,
                                    utf16_index: 11,
                                }
                            }
                        }))
                    })
                })
            },
            bindings: Vec::new()
        })
    )
}

#[test]
fn lambda_params_without_dot_at_the_ending() {
    let source_code = "\\x y z (w w) 3 5";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors, 
        &[
            "Token inesperado encontrado, esperava-se um \"<identificador>\", na linha 1 e coluna 8",
            "Token inesperado encontrado, esperava-se um \"<identificador>\", na linha 1 e coluna 12",
            "Token inesperado encontrado, esperava-se um \"<identificador>\", na linha 1 e coluna 14",
            "Token inesperado encontrado, esperava-se um \"<identificador>\", na linha 1 e coluna 16",
            "Fim inesperado do código"
        ]
    );

    assert_eq!(
        ast,
        None
    )
}