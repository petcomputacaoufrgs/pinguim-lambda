#![allow(unreachable_code)]

use super::{ast, parse};
use crate::compiler::lexer::generate_tokens;
use crate::compiler::{
    error::Diagnostics,
    position::{Position, Span},
};

/*
Testes caso de sucesso
- capacidade de parsear identificadores - feito
- capacidade de parsear numerais - feito
- capacidade de parsear lambdas - feito
- capacidade de parsear param lambdas aninhados- feito
- capacidade de "empilhar" aplicações corretamente - feito
- capacidade de balancear parentêsis - feito
- capacidade de parsear programas sem let - feito (englobado pelos outros)
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

#[test]
fn parse_numeral() {
    let source_code = "42";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Number(42),
            bindings: Vec::new(),
        })
    )
}

#[test]
fn parse_lambda() {
    let source_code = "\\foo. bar";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Lambda {
                parameter: ast::Symbol {
                    content: String::from("foo"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 2,
                            utf8_index: 1,
                            utf16_index: 1,
                        },
                        end: Position {
                            line: 1,
                            column: 5,
                            utf8_index: 4,
                            utf16_index: 4,
                        }
                    }
                },

                body: Box::new(ast::Expr::Variable(ast::Symbol {
                    content: String::from("bar"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 7,
                            utf8_index: 6,
                            utf16_index: 6,
                        },
                        end: Position {
                            line: 1,
                            column: 10,
                            utf8_index: 9,
                            utf16_index: 9,
                        }
                    }
                })),
            },
            bindings: Vec::new(),
        })
    )
}

#[test]
fn parse_lambda_with_many_params() {
    let source_code = "\\foo1 foo2 foo3. bar";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Lambda {
                parameter: ast::Symbol {
                    content: String::from("foo1"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 2,
                            utf8_index: 1,
                            utf16_index: 1,
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
                        content: String::from("foo2"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 7,
                                utf8_index: 6,
                                utf16_index: 6,
                            },
                            end: Position {
                                line: 1,
                                column: 11,
                                utf8_index: 10,
                                utf16_index: 10,
                            }
                        }
                    },

                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("foo3"),
                            span: Span {
                                start: Position {
                                    line: 1,
                                    column: 12,
                                    utf8_index: 11,
                                    utf16_index: 11,
                                },
                                end: Position {
                                    line: 1,
                                    column: 16,
                                    utf8_index: 15,
                                    utf16_index: 15,
                                }
                            }
                        },

                        body: Box::new(ast::Expr::Variable(ast::Symbol {
                            content: String::from("bar"),
                            span: Span {
                                start: Position {
                                    line: 1,
                                    column: 18,
                                    utf8_index: 17,
                                    utf16_index: 17,
                                },
                                end: Position {
                                    line: 1,
                                    column: 21,
                                    utf8_index: 20,
                                    utf16_index: 20,
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
fn parse_lambda_with_many_params_nested() {
    let source_code = "\\foo1. \\foo2. \\foo3. bar";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Lambda {
                parameter: ast::Symbol {
                    content: String::from("foo1"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 2,
                            utf8_index: 1,
                            utf16_index: 1,
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
                        content: String::from("foo2"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 9,
                                utf8_index: 8,
                                utf16_index: 8,
                            },
                            end: Position {
                                line: 1,
                                column: 13,
                                utf8_index: 12,
                                utf16_index: 12,
                            }
                        }
                    },

                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("foo3"),
                            span: Span {
                                start: Position {
                                    line: 1,
                                    column: 16,
                                    utf8_index: 15,
                                    utf16_index: 15,
                                },
                                end: Position {
                                    line: 1,
                                    column: 20,
                                    utf8_index: 19,
                                    utf16_index: 19,
                                }
                            }
                        },

                        body: Box::new(ast::Expr::Variable(ast::Symbol {
                            content: String::from("bar"),
                            span: Span {
                                start: Position {
                                    line: 1,
                                    column: 22,
                                    utf8_index: 21,
                                    utf16_index: 21,
                                },
                                end: Position {
                                    line: 1,
                                    column: 25,
                                    utf8_index: 24,
                                    utf16_index: 24,
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
fn parse_app() {
    let source_code = "fun arg0 arg1";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Application {
                function: Box::new(ast::Expr::Application {
                    function: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("fun"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 1,
                                utf8_index: 0,
                                utf16_index: 0,
                            },
                            end: Position {
                                line: 1,
                                column: 4,
                                utf8_index: 3,
                                utf16_index: 3,
                            }
                        }
                    })),
                    argument: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("arg0"),
                        span: Span {
                            start: Position {
                                line: 1,
                                column: 5,
                                utf8_index: 4,
                                utf16_index: 4,
                            },
                            end: Position {
                                line: 1,
                                column: 9,
                                utf8_index: 8,
                                utf16_index: 8,
                            }
                        }
                    })),
                }),
                argument: Box::new(ast::Expr::Variable(ast::Symbol {
                    content: String::from("arg1"),
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 10,
                            utf8_index: 9,
                            utf16_index: 9,
                        },
                        end: Position {
                            line: 1,
                            column: 14,
                            utf8_index: 13,
                            utf16_index: 13,
                        }
                    }
                })),
            },
            bindings: Vec::new(),
        })
    )
}

#[test]
fn parse_parenthesized() {
    let source_code = "f (g (\\x. g (x x)) (h a)) ((k) b)";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    // f
    let f_var = ast::Expr::Variable(ast::Symbol {
        content: String::from("f"),
        span: Span {
            start: Position {
                line: 1,
                column: 1,
                utf8_index: 0,
                utf16_index: 0,
            },
            end: Position { line: 1, column: 2, utf8_index: 1, utf16_index: 1 },
        },
    });

    // x x
    let app_x_to_x = ast::Expr::Application {
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
                },
            },
        })),

        argument: Box::new(ast::Expr::Variable(ast::Symbol {
            content: String::from("x"),
            span: Span {
                start: Position {
                    line: 1,
                    column: 16,
                    utf8_index: 15,
                    utf16_index: 15,
                },
                end: Position {
                    line: 1,
                    column: 17,
                    utf8_index: 16,
                    utf16_index: 16,
                },
            },
        })),
    };

    // (\x. g (x x))
    let lambda_x = ast::Expr::Lambda {
        parameter: ast::Symbol {
            content: String::from("x"),
            span: Span {
                start: Position {
                    line: 1,
                    column: 8,
                    utf8_index: 7,
                    utf16_index: 7,
                },
                end: Position {
                    line: 1,
                    column: 9,
                    utf8_index: 8,
                    utf16_index: 8,
                },
            },
        },
        body: Box::new(ast::Expr::Application {
            function: Box::new(ast::Expr::Variable(ast::Symbol {
                content: String::from("g"),
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
                    },
                },
            })),
            argument: Box::new(app_x_to_x),
        }),
    };

    // g (\x. g (x x))
    let app_lambda_to_g = ast::Expr::Application {
        function: Box::new(ast::Expr::Variable(ast::Symbol {
            content: String::from("g"),
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
                },
            },
        })),
        argument: Box::new(lambda_x),
    };

    // (h a)
    let app_a_to_h = ast::Expr::Application {
        function: Box::new(ast::Expr::Variable(ast::Symbol {
            content: String::from("h"),
            span: Span {
                start: Position {
                    line: 1,
                    column: 21,
                    utf8_index: 20,
                    utf16_index: 20,
                },
                end: Position {
                    line: 1,
                    column: 22,
                    utf8_index: 21,
                    utf16_index: 21,
                },
            },
        })),
        argument: Box::new(ast::Expr::Variable(ast::Symbol {
            content: String::from("a"),
            span: Span {
                start: Position {
                    line: 1,
                    column: 23,
                    utf8_index: 22,
                    utf16_index: 22,
                },
                end: Position {
                    line: 1,
                    column: 24,
                    utf8_index: 23,
                    utf16_index: 23,
                },
            },
        })),
    };

    // ((k) b)
    let app_b_to_k = ast::Expr::Application {
        function: Box::new(ast::Expr::Variable(ast::Symbol {
            content: String::from("k"),
            span: Span {
                start: Position {
                    line: 1,
                    column: 29,
                    utf8_index: 28,
                    utf16_index: 28,
                },
                end: Position {
                    line: 1,
                    column: 30,
                    utf8_index: 29,
                    utf16_index: 29,
                },
            },
        })),
        argument: Box::new(ast::Expr::Variable(ast::Symbol {
            content: String::from("b"),
            span: Span {
                start: Position {
                    line: 1,
                    column: 32,
                    utf8_index: 31,
                    utf16_index: 31,
                },
                end: Position {
                    line: 1,
                    column: 33,
                    utf8_index: 32,
                    utf16_index: 32,
                },
            },
        })),
    };

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Application {
                function: Box::new(ast::Expr::Application {
                    function: Box::new(f_var),
                    argument: Box::new(ast::Expr::Application {
                        function: Box::new(app_lambda_to_g),
                        argument: Box::new(app_a_to_h),
                    }),
                }),
                argument: Box::new(app_b_to_k),
            },
            bindings: Vec::new(),
        })
    )
}

#[test]
fn parse_with_one_binding() {
    let source_code = "let\ntwo = \\f x. f (f x);\nin\ntwo (two two)";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Application {
                function: Box::new(ast::Expr::Variable(ast::Symbol {
                    content: String::from("two"),
                    span: Span {
                        start: Position {
                            line: 4,
                            column: 1,
                            utf8_index: 28,
                            utf16_index: 28,
                        },
                        end: Position {
                            line: 4,
                            column: 4,
                            utf8_index: 31,
                            utf16_index: 31,
                        }
                    }
                })),
                argument: Box::new(ast::Expr::Application {
                    function: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("two"),
                        span: Span {
                            start: Position {
                                line: 4,
                                column: 6,
                                utf8_index: 33,
                                utf16_index: 33,
                            },
                            end: Position {
                                line: 4,
                                column: 9,
                                utf8_index: 36,
                                utf16_index: 36,
                            }
                        }
                    })),
                    argument: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("two"),
                        span: Span {
                            start: Position {
                                line: 4,
                                column: 10,
                                utf8_index: 37,
                                utf16_index: 37,
                            },
                            end: Position {
                                line: 4,
                                column: 13,
                                utf8_index: 40,
                                utf16_index: 40,
                            }
                        }
                    })),
                })
            },
            bindings: vec![ast::Binding {
                name: ast::Symbol {
                    content: String::from("two"),
                    span: Span {
                        start: Position {
                            line: 2,
                            column: 1,
                            utf8_index: 4,
                            utf16_index: 4,
                        },
                        end: Position {
                            line: 2,
                            column: 4,
                            utf8_index: 7,
                            utf16_index: 7,
                        }
                    }
                },
                expression: ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("f"),
                        span: Span {
                            start: Position {
                                line: 2,
                                column: 8,
                                utf8_index: 11,
                                utf16_index: 11,
                            },
                            end: Position {
                                line: 2,
                                column: 9,
                                utf8_index: 12,
                                utf16_index: 12,
                            }
                        }
                    },
                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("x"),
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
                        body: Box::new(ast::Expr::Application {
                            function: Box::new(ast::Expr::Variable(
                                ast::Symbol {
                                    content: String::from("f"),
                                    span: Span {
                                        start: Position {
                                            line: 2,
                                            column: 13,
                                            utf8_index: 16,
                                            utf16_index: 16,
                                        },
                                        end: Position {
                                            line: 2,
                                            column: 14,
                                            utf8_index: 17,
                                            utf16_index: 17,
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
                                                line: 2,
                                                column: 16,
                                                utf8_index: 19,
                                                utf16_index: 19,
                                            },
                                            end: Position {
                                                line: 2,
                                                column: 17,
                                                utf8_index: 20,
                                                utf16_index: 20,
                                            }
                                        }
                                    }
                                )),
                                argument: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("x"),
                                        span: Span {
                                            start: Position {
                                                line: 2,
                                                column: 18,
                                                utf8_index: 21,
                                                utf16_index: 21,
                                            },
                                            end: Position {
                                                line: 2,
                                                column: 19,
                                                utf8_index: 22,
                                                utf16_index: 22,
                                            }
                                        }
                                    }
                                )),
                            })
                        })
                    }),
                },
            }]
        })
    )
}

#[test]
fn parse_with_one_binding_no_semicolon() {
    let source_code = "let\ntwo = \\f x. f (f x)\nin\ntwo (two two)";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            main_expression: ast::Expr::Application {
                function: Box::new(ast::Expr::Variable(ast::Symbol {
                    content: String::from("two"),
                    span: Span {
                        start: Position {
                            line: 4,
                            column: 1,
                            utf8_index: 27,
                            utf16_index: 27,
                        },
                        end: Position {
                            line: 4,
                            column: 4,
                            utf8_index: 30,
                            utf16_index: 30,
                        }
                    }
                })),
                argument: Box::new(ast::Expr::Application {
                    function: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("two"),
                        span: Span {
                            start: Position {
                                line: 4,
                                column: 6,
                                utf8_index: 32,
                                utf16_index: 32,
                            },
                            end: Position {
                                line: 4,
                                column: 9,
                                utf8_index: 35,
                                utf16_index: 35,
                            }
                        }
                    })),
                    argument: Box::new(ast::Expr::Variable(ast::Symbol {
                        content: String::from("two"),
                        span: Span {
                            start: Position {
                                line: 4,
                                column: 10,
                                utf8_index: 36,
                                utf16_index: 36,
                            },
                            end: Position {
                                line: 4,
                                column: 13,
                                utf8_index: 39,
                                utf16_index: 39,
                            }
                        }
                    })),
                })
            },
            bindings: vec![ast::Binding {
                name: ast::Symbol {
                    content: String::from("two"),
                    span: Span {
                        start: Position {
                            line: 2,
                            column: 1,
                            utf8_index: 4,
                            utf16_index: 4,
                        },
                        end: Position {
                            line: 2,
                            column: 4,
                            utf8_index: 7,
                            utf16_index: 7,
                        }
                    }
                },
                expression: ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("f"),
                        span: Span {
                            start: Position {
                                line: 2,
                                column: 8,
                                utf8_index: 11,
                                utf16_index: 11,
                            },
                            end: Position {
                                line: 2,
                                column: 9,
                                utf8_index: 12,
                                utf16_index: 12,
                            }
                        }
                    },
                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("x"),
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
                        body: Box::new(ast::Expr::Application {
                            function: Box::new(ast::Expr::Variable(
                                ast::Symbol {
                                    content: String::from("f"),
                                    span: Span {
                                        start: Position {
                                            line: 2,
                                            column: 13,
                                            utf8_index: 16,
                                            utf16_index: 16,
                                        },
                                        end: Position {
                                            line: 2,
                                            column: 14,
                                            utf8_index: 17,
                                            utf16_index: 17,
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
                                                line: 2,
                                                column: 16,
                                                utf8_index: 19,
                                                utf16_index: 19,
                                            },
                                            end: Position {
                                                line: 2,
                                                column: 17,
                                                utf8_index: 20,
                                                utf16_index: 20,
                                            }
                                        }
                                    }
                                )),
                                argument: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("x"),
                                        span: Span {
                                            start: Position {
                                                line: 2,
                                                column: 18,
                                                utf8_index: 21,
                                                utf16_index: 21,
                                            },
                                            end: Position {
                                                line: 2,
                                                column: 19,
                                                utf8_index: 22,
                                                utf16_index: 22,
                                            }
                                        }
                                    }
                                )),
                            })
                        })
                    }),
                },
            }]
        })
    )
}

#[test]
fn parse_with_many_bindings() {
    let source_code = "let\nsucc = \\n. \\f x. n f (f x);\nadd = \\m n. m succ n;\nmul = \\m n. m (add n) 0;\n\nin\nmul 3 5";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);

    assert!(diagnostics.is_ok());
    assert_eq!(
        ast,
        Some(ast::Program {
            bindings: vec![
                ast::Binding {
                    name: ast::Symbol {
                        content: String::from("succ"),
                        span: Span {
                            start: Position {
                                line: 2,
                                column: 1,
                                utf8_index: 4,
                                utf16_index: 4,
                            },
                            end: Position {
                                line: 2,
                                column: 5,
                                utf8_index: 8,
                                utf16_index: 8,
                            }
                        }
                    },
                    expression: ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("n"),
                            span: Span {
                                start: Position {
                                    line: 2,
                                    column: 9,
                                    utf8_index: 12,
                                    utf16_index: 12,
                                },
                                end: Position {
                                    line: 2,
                                    column: 10,
                                    utf8_index: 13,
                                    utf16_index: 13,
                                }
                            }
                        },
                        body: Box::new(ast::Expr::Lambda {
                            parameter: ast::Symbol {
                                content: String::from("f"),
                                span: Span {
                                    start: Position {
                                        line: 2,
                                        column: 13,
                                        utf8_index: 16,
                                        utf16_index: 16,
                                    },
                                    end: Position {
                                        line: 2,
                                        column: 14,
                                        utf8_index: 17,
                                        utf16_index: 17,
                                    }
                                }
                            },
                            body: Box::new(ast::Expr::Lambda {
                                parameter: ast::Symbol {
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
                                },
                                body: Box::new(ast::Expr::Application {
                                    function: Box::new(
                                        ast::Expr::Application {
                                            function: Box::new(
                                                ast::Expr::Variable(
                                                    ast::Symbol {
                                                        content: String::from(
                                                            "n"
                                                        ),
                                                        span: Span {
                                                            start: Position {
                                                                line: 2,
                                                                column: 18,
                                                                utf8_index: 21,
                                                                utf16_index: 21,
                                                            },
                                                            end: Position {
                                                                line: 2,
                                                                column: 19,
                                                                utf8_index: 22,
                                                                utf16_index: 22,
                                                            }
                                                        }
                                                    }
                                                )
                                            ),
                                            argument: Box::new(
                                                ast::Expr::Variable(
                                                    ast::Symbol {
                                                        content: String::from(
                                                            "f"
                                                        ),
                                                        span: Span {
                                                            start: Position {
                                                                line: 2,
                                                                column: 20,
                                                                utf8_index: 23,
                                                                utf16_index: 23,
                                                            },
                                                            end: Position {
                                                                line: 2,
                                                                column: 21,
                                                                utf8_index: 24,
                                                                utf16_index: 24,
                                                            }
                                                        }
                                                    }
                                                )
                                            ),
                                        }
                                    ),
                                    argument: Box::new(
                                        ast::Expr::Application {
                                            function: Box::new(
                                                ast::Expr::Variable(
                                                    ast::Symbol {
                                                        content: String::from(
                                                            "f"
                                                        ),
                                                        span: Span {
                                                            start: Position {
                                                                line: 2,
                                                                column: 23,
                                                                utf8_index: 26,
                                                                utf16_index: 26,
                                                            },
                                                            end: Position {
                                                                line: 2,
                                                                column: 24,
                                                                utf8_index: 27,
                                                                utf16_index: 27,
                                                            }
                                                        }
                                                    }
                                                )
                                            ),
                                            argument: Box::new(
                                                ast::Expr::Variable(
                                                    ast::Symbol {
                                                        content: String::from(
                                                            "x"
                                                        ),
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
                                                            }
                                                        }
                                                    }
                                                )
                                            ),
                                        }
                                    ),
                                }),
                            }),
                        })
                    },
                },
                // "let\nsucc = \\n. \\f x. n f (f x);\nadd = \\m n. m succ n;\nmul = \\m n. m (add n) 0;\n\nin\nmul 3 5"
                // corrigir os positions a partir daqui
                ast::Binding {
                    name: ast::Symbol {
                        content: String::from("add"),
                        span: Span {
                            start: Position {
                                line: 2,
                                column: 1,
                                utf8_index: 4,
                                utf16_index: 4,
                            },
                            end: Position {
                                line: 2,
                                column: 5,
                                utf8_index: 8,
                                utf16_index: 8,
                            }
                        }
                    },
                    expression: ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("m"),
                            span: Span {
                                start: Position {
                                    line: 2,
                                    column: 1,
                                    utf8_index: 4,
                                    utf16_index: 4,
                                },
                                end: Position {
                                    line: 2,
                                    column: 5,
                                    utf8_index: 8,
                                    utf16_index: 8,
                                }
                            }
                        },
                        body: Box::new(ast::Expr::Lambda {
                            parameter: ast::Symbol {
                                content: String::from("n"),
                                span: Span {
                                    start: Position {
                                        line: 2,
                                        column: 1,
                                        utf8_index: 4,
                                        utf16_index: 4,
                                    },
                                    end: Position {
                                        line: 2,
                                        column: 5,
                                        utf8_index: 8,
                                        utf16_index: 8,
                                    }
                                }
                            },
                            body: Box::new(ast::Expr::Application {
                                function: Box::new(ast::Expr::Application {
                                    function: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("m"),
                                            span: Span {
                                                start: Position {
                                                    line: 2,
                                                    column: 1,
                                                    utf8_index: 4,
                                                    utf16_index: 4,
                                                },
                                                end: Position {
                                                    line: 2,
                                                    column: 5,
                                                    utf8_index: 8,
                                                    utf16_index: 8,
                                                }
                                            }
                                        }
                                    )),
                                    argument: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("succ"),
                                            span: Span {
                                                start: Position {
                                                    line: 2,
                                                    column: 1,
                                                    utf8_index: 4,
                                                    utf16_index: 4,
                                                },
                                                end: Position {
                                                    line: 2,
                                                    column: 5,
                                                    utf8_index: 8,
                                                    utf16_index: 8,
                                                }
                                            }
                                        }
                                    )),
                                }),
                                argument: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("n"),
                                        span: Span {
                                            start: Position {
                                                line: 2,
                                                column: 1,
                                                utf8_index: 4,
                                                utf16_index: 4,
                                            },
                                            end: Position {
                                                line: 2,
                                                column: 5,
                                                utf8_index: 8,
                                                utf16_index: 8,
                                            }
                                        }
                                    }
                                ))
                            })
                        })
                    }
                }
            ],
            main_expression: todo!(),
        }),
    )
}
