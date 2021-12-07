use super::{
    super::{
        error::Diagnostics,
        position::{Position, Span},
    },
    generate_tokens,
    token::{Token, TokenType},
};

#[test]
fn empty_src() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(tokens, Vec::new());
}

#[test]
fn token_lambda() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("\\", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Lambda,
            content: "\\".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
            },
        }]
    );
}

#[test]
fn token_identifier() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("lambda", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Identifier,
            content: "lambda".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 7,
                    utf8_index: 6,
                    utf16_index: 6,
                },
            },
        }]
    );
}

#[test]
fn token_number() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("31156", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Number,
            content: "31156".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 6,
                    utf8_index: 5,
                    utf16_index: 5,
                },
            },
        }]
    );
}

#[test]
fn token_equal() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("=", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Equal,
            content: "=".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
            },
        }]
    );
}

#[test]
fn token_let() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("let", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Let,
            content: "let".to_owned(),
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
                },
            },
        }]
    );
}

#[test]
fn token_in() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("in", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::In,
            content: "in".to_owned(),
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
                },
            },
        }]
    );
}

#[test]
fn token_dot() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(".", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Dot,
            content: ".".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
            },
        }]
    );
}

#[test]
fn token_open_paren() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("(", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::OpenParen,
            content: "(".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
            },
        }]
    );
}

#[test]
fn token_close_paren() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(")", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::CloseParen,
            content: ")".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
            },
        }]
    );
}

#[test]
fn code_without_keywords() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" \\a b. a ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Lambda,
            content: "\\".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "a".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "b".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Dot,
            content: ".".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "a".to_owned(),
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
        }]
    );
}

#[test]
fn code_with_keyword_let() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("let\n    true = \\a b. a ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Let,
            content: "let".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "true".to_owned(),
            span: Span {
                start: Position {
                    line: 2,
                    column: 5,
                    utf8_index: 8,
                    utf16_index: 8,
                },
                end: Position {
                    line: 2,
                    column: 9,
                    utf8_index: 12,
                    utf16_index: 12,
                },
            },
        },
        Token {
            token_type: TokenType::Equal,
            content: "=".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Lambda,
            content: "\\".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "a".to_owned(),
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
        Token {
            token_type: TokenType::Identifier,
            content: "b".to_owned(),
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
        Token {
            token_type: TokenType::Dot,
            content: ".".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "a".to_owned(),
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
        }]
    );
}

#[test]
fn code_with_comments() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("let\n    -- função para valor true\n    true = \\a b. a ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Let,
            content: "let".to_owned(),
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
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "true".to_owned(),
            span: Span {
                start: Position {
                    line: 3,
                    column: 5,
                    utf8_index: 40,
                    utf16_index: 38,
                },
                end: Position {
                    line: 3,
                    column: 9,
                    utf8_index: 44,
                    utf16_index: 42,
                },
            },
        },
        Token {
            token_type: TokenType::Equal,
            content: "=".to_owned(),
            span: Span {
                start: Position {
                    line: 3,
                    column: 10,
                    utf8_index: 45,
                    utf16_index: 43,
                },
                end: Position {
                    line: 3,
                    column: 11,
                    utf8_index: 46,
                    utf16_index: 44,
                },
            },
        },
        Token {
            token_type: TokenType::Lambda,
            content: "\\".to_owned(),
            span: Span {
                start: Position {
                    line: 3,
                    column: 12,
                    utf8_index: 47,
                    utf16_index: 45,
                },
                end: Position {
                    line: 3,
                    column: 13,
                    utf8_index: 48,
                    utf16_index: 46,
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "a".to_owned(),
            span: Span {
                start: Position {
                    line: 3,
                    column: 13,
                    utf8_index: 48,
                    utf16_index: 46,
                },
                end: Position {
                    line: 3,
                    column: 14,
                    utf8_index: 49,
                    utf16_index: 47,
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "b".to_owned(),
            span: Span {
                start: Position {
                    line: 3,
                    column: 15,
                    utf8_index: 50,
                    utf16_index: 48,
                },
                end: Position {
                    line: 3,
                    column: 16,
                    utf8_index: 51,
                    utf16_index: 49,
                },
            },
        },
        Token {
            token_type: TokenType::Dot,
            content: ".".to_owned(),
            span: Span {
                start: Position {
                    line: 3,
                    column: 16,
                    utf8_index: 51,
                    utf16_index: 49,
                },
                end: Position {
                    line: 3,
                    column: 17,
                    utf8_index: 52,
                    utf16_index: 50,
                },
            },
        },
        Token {
            token_type: TokenType::Identifier,
            content: "a".to_owned(),
            span: Span {
                start: Position {
                    line: 3,
                    column: 18,
                    utf8_index: 53,
                    utf16_index: 51,
                },
                end: Position {
                    line: 3,
                    column: 19,
                    utf8_index: 54,
                    utf16_index: 52,
                },
            },
        }]
    );
}

#[test]
fn many_errors() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("let?\n    - função para valor true\n    t:rue = \\a b. a ", &mut diagnostics);
    assert!(diagnostics.is_err());

    let errors = diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &[
            "Caracter '?' é inválido, na linha 1 e coluna 4",
            "Começo inválido de comentário, na linha 2 e coluna 5",
            "Caracter ':' é inválido, na linha 3 e coluna 6",
        ]
    );

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Let,
                content: "let".to_owned(),
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
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "t".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 5,
                        utf8_index: 40,
                        utf16_index: 38,
                    },
                    end: Position {
                        line: 3,
                        column: 6,
                        utf8_index: 41,
                        utf16_index: 39,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "rue".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 7,
                        utf8_index: 42,
                        utf16_index: 40,
                    },
                    end: Position {
                        line: 3,
                        column: 10,
                        utf8_index: 45,
                        utf16_index: 43,
                    },
                },
            },
            Token {
                token_type: TokenType::Equal,
                content: "=".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 11,
                        utf8_index: 46,
                        utf16_index: 44,
                    },
                    end: Position {
                        line: 3,
                        column: 12,
                        utf8_index: 47,
                        utf16_index: 45,
                    },
                },
            },
            Token {
                token_type: TokenType::Lambda,
                content: "\\".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 13,
                        utf8_index: 48,
                        utf16_index: 46,
                    },
                    end: Position {
                        line: 3,
                        column: 14,
                        utf8_index: 49,
                        utf16_index: 47,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "a".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 14,
                        utf8_index: 49,
                        utf16_index: 47,
                    },
                    end: Position {
                        line: 3,
                        column: 15,
                        utf8_index: 50,
                        utf16_index: 48,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "b".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 16,
                        utf8_index: 51,
                        utf16_index: 49,
                    },
                    end: Position {
                        line: 3,
                        column: 17,
                        utf8_index: 52,
                        utf16_index: 50,
                    },
                },
            },
            Token {
                token_type: TokenType::Dot,
                content: ".".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 17,
                        utf8_index: 52,
                        utf16_index: 50,
                    },
                    end: Position {
                        line: 3,
                        column: 18,
                        utf8_index: 53,
                        utf16_index: 51,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "a".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 19,
                        utf8_index: 54,
                        utf16_index: 52,
                    },
                    end: Position {
                        line: 3,
                        column: 20,
                        utf8_index: 55,
                        utf16_index: 53,
                    },
                },
            }
        ]
    )
}

#[test]
fn tokens_together() {
    let mut diagnostics = Diagnostics::new();
    let code = "\\x.x";
    let tokens = generate_tokens(code, &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens, 
        &[
            Token {
                token_type: TokenType::Lambda,
                content: "\\".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        utf8_index: 0,
                        utf16_index: 0,
                    },
                    end: Position {
                        line: 1,
                        column: 2,
                        utf8_index: 1,
                        utf16_index: 1,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "x".to_owned(),
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
                    },
                },
            },
            Token {
                token_type: TokenType::Dot,
                content: ".".to_owned(),
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
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "x".to_owned(),
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
            }            
        ]);
}