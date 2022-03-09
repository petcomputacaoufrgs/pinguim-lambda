use crate::compiler::lexer::generate_tokens;
use crate::compiler::parser::{ast, parse};
use pinguim_language::{
    error::Diagnostics,
    position::{Position, Span},
};

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
    let source_code = "let\nsucc = \\n. \\f x. n f (f x);\nadd = \\m n. m succ n;\nmul = \\m n. m (add n) 0;\nin\nmul 3 5";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics);
    let expected_ast = ast::Program {
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
                        },
                    },
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
                            },
                        },
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
                                },
                            },
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
                                    },
                                },
                            },
                            body: Box::new(ast::Expr::Application {
                                function: Box::new(ast::Expr::Application {
                                    function: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("n"),
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
                                                },
                                            },
                                        },
                                    )),
                                    argument: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("f"),
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
                                                },
                                            },
                                        },
                                    )),
                                }),
                                argument: Box::new(ast::Expr::Application {
                                    function: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("f"),
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
                                }),
                            }),
                        }),
                    }),
                },
            },
            ast::Binding {
                name: ast::Symbol {
                    content: String::from("add"),
                    span: Span {
                        start: Position {
                            line: 3,
                            column: 1,
                            utf8_index: 32,
                            utf16_index: 32,
                        },
                        end: Position {
                            line: 3,
                            column: 4,
                            utf8_index: 35,
                            utf16_index: 35,
                        },
                    },
                },
                expression: ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("m"),
                        span: Span {
                            start: Position {
                                line: 3,
                                column: 8,
                                utf8_index: 39,
                                utf16_index: 39,
                            },
                            end: Position {
                                line: 3,
                                column: 9,
                                utf8_index: 40,
                                utf16_index: 40,
                            },
                        },
                    },
                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("n"),
                            span: Span {
                                start: Position {
                                    line: 3,
                                    column: 10,
                                    utf8_index: 41,
                                    utf16_index: 41,
                                },
                                end: Position {
                                    line: 3,
                                    column: 11,
                                    utf8_index: 42,
                                    utf16_index: 42,
                                },
                            },
                        },
                        body: Box::new(ast::Expr::Application {
                            function: Box::new(ast::Expr::Application {
                                function: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("m"),
                                        span: Span {
                                            start: Position {
                                                line: 3,
                                                column: 13,
                                                utf8_index: 44,
                                                utf16_index: 44,
                                            },
                                            end: Position {
                                                line: 3,
                                                column: 14,
                                                utf8_index: 45,
                                                utf16_index: 45,
                                            },
                                        },
                                    },
                                )),
                                argument: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("succ"),
                                        span: Span {
                                            start: Position {
                                                line: 3,
                                                column: 15,
                                                utf8_index: 46,
                                                utf16_index: 46,
                                            },
                                            end: Position {
                                                line: 3,
                                                column: 19,
                                                utf8_index: 50,
                                                utf16_index: 50,
                                            },
                                        },
                                    },
                                )),
                            }),
                            argument: Box::new(ast::Expr::Variable(
                                ast::Symbol {
                                    content: String::from("n"),
                                    span: Span {
                                        start: Position {
                                            line: 3,
                                            column: 20,
                                            utf8_index: 51,
                                            utf16_index: 51,
                                        },
                                        end: Position {
                                            line: 3,
                                            column: 21,
                                            utf8_index: 52,
                                            utf16_index: 52,
                                        },
                                    },
                                },
                            )),
                        }),
                    }),
                },
            },
            ast::Binding {
                name: ast::Symbol {
                    content: String::from("mul"),
                    span: Span {
                        start: Position {
                            line: 4,
                            column: 1,
                            utf8_index: 54,
                            utf16_index: 54,
                        },
                        end: Position {
                            line: 4,
                            column: 4,
                            utf8_index: 57,
                            utf16_index: 57,
                        },
                    },
                },
                expression: ast::Expr::Lambda {
                    parameter: ast::Symbol {
                        content: String::from("m"),
                        span: Span {
                            start: Position {
                                line: 4,
                                column: 8,
                                utf8_index: 61,
                                utf16_index: 61,
                            },
                            end: Position {
                                line: 4,
                                column: 9,
                                utf8_index: 62,
                                utf16_index: 62,
                            },
                        },
                    },
                    body: Box::new(ast::Expr::Lambda {
                        parameter: ast::Symbol {
                            content: String::from("n"),
                            span: Span {
                                start: Position {
                                    line: 4,
                                    column: 10,
                                    utf8_index: 63,
                                    utf16_index: 63,
                                },
                                end: Position {
                                    line: 4,
                                    column: 11,
                                    utf8_index: 64,
                                    utf16_index: 64,
                                },
                            },
                        },
                        body: Box::new(ast::Expr::Application {
                            function: Box::new(ast::Expr::Application {
                                function: Box::new(ast::Expr::Variable(
                                    ast::Symbol {
                                        content: String::from("m"),
                                        span: Span {
                                            start: Position {
                                                line: 4,
                                                column: 13,
                                                utf8_index: 66,
                                                utf16_index: 66,
                                            },
                                            end: Position {
                                                line: 4,
                                                column: 14,
                                                utf8_index: 67,
                                                utf16_index: 67,
                                            },
                                        },
                                    },
                                )),
                                argument: Box::new(ast::Expr::Application {
                                    function: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("add"),
                                            span: Span {
                                                start: Position {
                                                    line: 4,
                                                    column: 16,
                                                    utf8_index: 69,
                                                    utf16_index: 69,
                                                },
                                                end: Position {
                                                    line: 4,
                                                    column: 19,
                                                    utf8_index: 72,
                                                    utf16_index: 72,
                                                },
                                            },
                                        },
                                    )),
                                    argument: Box::new(ast::Expr::Variable(
                                        ast::Symbol {
                                            content: String::from("n"),
                                            span: Span {
                                                start: Position {
                                                    line: 4,
                                                    column: 20,
                                                    utf8_index: 73,
                                                    utf16_index: 73,
                                                },
                                                end: Position {
                                                    line: 4,
                                                    column: 21,
                                                    utf8_index: 74,
                                                    utf16_index: 74,
                                                },
                                            },
                                        },
                                    )),
                                }),
                            }),
                            argument: Box::new(ast::Expr::Number(0)),
                        }),
                    }),
                },
            },
        ],
        main_expression: ast::Expr::Application {
            function: Box::new(ast::Expr::Application {
                function: Box::new(ast::Expr::Variable(ast::Symbol {
                    content: String::from("mul"),
                    span: Span {
                        start: Position {
                            line: 6,
                            column: 1,
                            utf8_index: 82,
                            utf16_index: 82,
                        },
                        end: Position {
                            line: 6,
                            column: 4,
                            utf8_index: 85,
                            utf16_index: 85,
                        },
                    },
                })),
                argument: Box::new(ast::Expr::Number(3)),
            }),
            argument: Box::new(ast::Expr::Number(5)),
        },
    };

    assert!(diagnostics.is_ok());
    assert_eq!(ast, Some(expected_ast));
}
