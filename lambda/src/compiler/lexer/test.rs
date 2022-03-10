use super::{
    generate_tokens,
    token::{Token, TokenType},
};
use pinguim_language::{
    error::Diagnostics,
    position::{Position, Span},
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
fn token_semicolon() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(";", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Semicolon,
            content: ";".to_owned(),
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
            }
        }]
    )
}

fn token_lambda_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("  \\  ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Lambda,
            content: "\\".to_owned(),
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
        }]
    );
}

#[test]
fn token_identifier_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("   lambda    ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Identifier,
            content: "lambda".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 4,
                    utf8_index: 3,
                    utf16_index: 3,
                },
                end: Position {
                    line: 1,
                    column: 10,
                    utf8_index: 9,
                    utf16_index: 9,
                },
            },
        }]
    );
}

#[test]
fn token_number_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" 31156 ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Number,
            content: "31156".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
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
fn token_equal_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("  =  ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Equal,
            content: "=".to_owned(),
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
        }]
    );
}

#[test]
fn token_let_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("  let ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Let,
            content: "let".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 3,
                    utf8_index: 2,
                    utf16_index: 2,
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
fn token_in_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" in   ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::In,
            content: "in".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
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
fn token_dot_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" .   ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Dot,
            content: ".".to_owned(),
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
        }]
    );
}

#[test]
fn token_open_paren_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" ( ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::OpenParen,
            content: "(".to_owned(),
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
        }]
    );
}

#[test]
fn token_close_paren_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("   )  ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::CloseParen,
            content: ")".to_owned(),
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
        }]
    );
}

#[test]
fn token_semicolon_with_spaces() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("  ;   ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Semicolon,
            content: ";".to_owned(),
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
            }
        }]
    )
}

#[test]
fn code_without_keywords() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" \\a b. a ", &mut diagnostics);
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
            }
        ]
    );
}

#[test]
fn code_with_keyword_let() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("let\n    true = \\a b. a ", &mut diagnostics);
    assert!(diagnostics.is_ok());
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
            }
        ]
    );
}

#[test]
fn code_with_comments() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(
        "let\n    -- função para valor true\n    true = \\a b. a ",
        &mut diagnostics,
    );
    assert!(diagnostics.is_ok());
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
            }
        ]
    );
}

#[test]
fn many_errors() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(
        "let?\n    - função para valor true\n    t:rue = \\a b. a ",
        &mut diagnostics,
    );
    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

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
fn bad_comment_start() {
    let mut diagnostics = Diagnostics::new();
    let code = "--começo do código\nlet\n - comentario mal formatado";
    let tokens = generate_tokens(code, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &["Começo inválido de comentário, na linha 3 e coluna 2",]
    );

    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Let,
            content: "let".to_owned(),
            span: Span {
                start: Position {
                    line: 2,
                    column: 1,
                    utf8_index: 21,
                    utf16_index: 19,
                },
                end: Position {
                    line: 2,
                    column: 4,
                    utf8_index: 24,
                    utf16_index: 22,
                },
            },
        }]
    );
}

#[test]
fn invalid_char() {
    let mut diagnostics = Diagnostics::new();
    let code = "@dd 2 3";
    let tokens = generate_tokens(code, &mut diagnostics);

    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Caracter '@' é inválido, na linha 1 e coluna 1",]);

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Identifier,
                content: "dd".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 2,
                        utf8_index: 1,
                        utf16_index: 1,
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
                token_type: TokenType::Number,
                content: "2".to_owned(),
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
                token_type: TokenType::Number,
                content: "3".to_owned(),
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
                    },
                },
            }
        ]
    );
}

#[test]
fn tokens_together() {
    let mut diagnostics = Diagnostics::new();
    let code = "let;in\\a=my_y.39(my_x)";
    let tokens = generate_tokens(code, &mut diagnostics);
    assert!(diagnostics.is_ok());
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
                token_type: TokenType::Semicolon,
                content: ";".to_owned(),
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
            },
            Token {
                token_type: TokenType::In,
                content: "in".to_owned(),
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
                    },
                },
            },
            Token {
                token_type: TokenType::Lambda,
                content: "\\".to_owned(),
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
            },
            Token {
                token_type: TokenType::Equal,
                content: "=".to_owned(),
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
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "my_y".to_owned(),
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
                    },
                },
            },
            Token {
                token_type: TokenType::Dot,
                content: ".".to_owned(),
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
            },
            Token {
                token_type: TokenType::Number,
                content: "39".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 15,
                        utf8_index: 14,
                        utf16_index: 14,
                    },
                    end: Position {
                        line: 1,
                        column: 17,
                        utf8_index: 16,
                        utf16_index: 16,
                    },
                },
            },
            Token {
                token_type: TokenType::OpenParen,
                content: "(".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 17,
                        utf8_index: 16,
                        utf16_index: 16,
                    },
                    end: Position {
                        line: 1,
                        column: 18,
                        utf8_index: 17,
                        utf16_index: 17,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "my_x".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 18,
                        utf8_index: 17,
                        utf16_index: 17,
                    },
                    end: Position {
                        line: 1,
                        column: 22,
                        utf8_index: 21,
                        utf16_index: 21,
                    },
                },
            },
            Token {
                token_type: TokenType::CloseParen,
                content: ")".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 22,
                        utf8_index: 21,
                        utf16_index: 21,
                    },
                    end: Position {
                        line: 1,
                        column: 23,
                        utf8_index: 22,
                        utf16_index: 22,
                    },
                },
            },
        ]
    );
}

#[test]
fn tokens_separated_by_a_lot_of_spaces() {
    let mut diagnostics = Diagnostics::new();
    let code = "let ;\nin \\ a = my_y . 39 ( my_x )";
    let tokens = generate_tokens(code, &mut diagnostics);
    assert!(diagnostics.is_ok());
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
                token_type: TokenType::Semicolon,
                content: ";".to_owned(),
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
                token_type: TokenType::In,
                content: "in".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 1,
                        utf8_index: 6,
                        utf16_index: 6,
                    },
                    end: Position {
                        line: 2,
                        column: 3,
                        utf8_index: 8,
                        utf16_index: 8,
                    },
                },
            },
            Token {
                token_type: TokenType::Lambda,
                content: "\\".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 4,
                        utf8_index: 9,
                        utf16_index: 9,
                    },
                    end: Position {
                        line: 2,
                        column: 5,
                        utf8_index: 10,
                        utf16_index: 10,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "a".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 6,
                        utf8_index: 11,
                        utf16_index: 11,
                    },
                    end: Position {
                        line: 2,
                        column: 7,
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
                        column: 8,
                        utf8_index: 13,
                        utf16_index: 13,
                    },
                    end: Position {
                        line: 2,
                        column: 9,
                        utf8_index: 14,
                        utf16_index: 14,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "my_y".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 10,
                        utf8_index: 15,
                        utf16_index: 15,
                    },
                    end: Position {
                        line: 2,
                        column: 14,
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
                        column: 15,
                        utf8_index: 20,
                        utf16_index: 20,
                    },
                    end: Position {
                        line: 2,
                        column: 16,
                        utf8_index: 21,
                        utf16_index: 21,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "39".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 17,
                        utf8_index: 22,
                        utf16_index: 22,
                    },
                    end: Position {
                        line: 2,
                        column: 19,
                        utf8_index: 24,
                        utf16_index: 24,
                    },
                },
            },
            Token {
                token_type: TokenType::OpenParen,
                content: "(".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 20,
                        utf8_index: 25,
                        utf16_index: 25,
                    },
                    end: Position {
                        line: 2,
                        column: 21,
                        utf8_index: 26,
                        utf16_index: 26,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "my_x".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 22,
                        utf8_index: 27,
                        utf16_index: 27,
                    },
                    end: Position {
                        line: 2,
                        column: 26,
                        utf8_index: 31,
                        utf16_index: 31,
                    },
                },
            },
            Token {
                token_type: TokenType::CloseParen,
                content: ")".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 27,
                        utf8_index: 32,
                        utf16_index: 32,
                    },
                    end: Position {
                        line: 2,
                        column: 28,
                        utf8_index: 33,
                        utf16_index: 33,
                    },
                },
            },
        ]
    )
}

#[test]
fn complete_code() {
    let mut diagnostics = Diagnostics::new();
    let code = "let\n   -- sucessor\n   succ = \\n. \\f x. n f (f x);\n   -- adição de naturais\n   add = \\n m. n succ m\nin add 2 3";
    let tokens = generate_tokens(code, &mut diagnostics);

    assert!(diagnostics.is_ok());
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
                content: "succ".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 4,
                        utf8_index: 22,
                        utf16_index: 22,
                    },
                    end: Position {
                        line: 3,
                        column: 8,
                        utf8_index: 26,
                        utf16_index: 26,
                    },
                },
            },
            Token {
                token_type: TokenType::Equal,
                content: "=".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 9,
                        utf8_index: 27,
                        utf16_index: 27,
                    },
                    end: Position {
                        line: 3,
                        column: 10,
                        utf8_index: 28,
                        utf16_index: 28,
                    },
                },
            },
            Token {
                token_type: TokenType::Lambda,
                content: "\\".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 11,
                        utf8_index: 29,
                        utf16_index: 29,
                    },
                    end: Position {
                        line: 3,
                        column: 12,
                        utf8_index: 30,
                        utf16_index: 30,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "n".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 12,
                        utf8_index: 30,
                        utf16_index: 30,
                    },
                    end: Position {
                        line: 3,
                        column: 13,
                        utf8_index: 31,
                        utf16_index: 31,
                    },
                },
            },
            Token {
                token_type: TokenType::Dot,
                content: ".".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 13,
                        utf8_index: 31,
                        utf16_index: 31,
                    },
                    end: Position {
                        line: 3,
                        column: 14,
                        utf8_index: 32,
                        utf16_index: 32,
                    },
                },
            },
            Token {
                token_type: TokenType::Lambda,
                content: "\\".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 15,
                        utf8_index: 33,
                        utf16_index: 33,
                    },
                    end: Position {
                        line: 3,
                        column: 16,
                        utf8_index: 34,
                        utf16_index: 34,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "f".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 16,
                        utf8_index: 34,
                        utf16_index: 34,
                    },
                    end: Position {
                        line: 3,
                        column: 17,
                        utf8_index: 35,
                        utf16_index: 35,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "x".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 18,
                        utf8_index: 36,
                        utf16_index: 36,
                    },
                    end: Position {
                        line: 3,
                        column: 19,
                        utf8_index: 37,
                        utf16_index: 37,
                    },
                },
            },
            Token {
                token_type: TokenType::Dot,
                content: ".".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 19,
                        utf8_index: 37,
                        utf16_index: 37,
                    },
                    end: Position {
                        line: 3,
                        column: 20,
                        utf8_index: 38,
                        utf16_index: 38,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "n".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 21,
                        utf8_index: 39,
                        utf16_index: 39,
                    },
                    end: Position {
                        line: 3,
                        column: 22,
                        utf8_index: 40,
                        utf16_index: 40,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "f".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 23,
                        utf8_index: 41,
                        utf16_index: 41,
                    },
                    end: Position {
                        line: 3,
                        column: 24,
                        utf8_index: 42,
                        utf16_index: 42,
                    },
                },
            },
            Token {
                token_type: TokenType::OpenParen,
                content: "(".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 25,
                        utf8_index: 43,
                        utf16_index: 43,
                    },
                    end: Position {
                        line: 3,
                        column: 26,
                        utf8_index: 44,
                        utf16_index: 44,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "f".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 26,
                        utf8_index: 44,
                        utf16_index: 44,
                    },
                    end: Position {
                        line: 3,
                        column: 27,
                        utf8_index: 45,
                        utf16_index: 45,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "x".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 28,
                        utf8_index: 46,
                        utf16_index: 46,
                    },
                    end: Position {
                        line: 3,
                        column: 29,
                        utf8_index: 47,
                        utf16_index: 47,
                    },
                },
            },
            Token {
                token_type: TokenType::CloseParen,
                content: ")".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 29,
                        utf8_index: 47,
                        utf16_index: 47,
                    },
                    end: Position {
                        line: 3,
                        column: 30,
                        utf8_index: 48,
                        utf16_index: 48,
                    },
                },
            },
            Token {
                token_type: TokenType::Semicolon,
                content: ";".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 30,
                        utf8_index: 48,
                        utf16_index: 48,
                    },
                    end: Position {
                        line: 3,
                        column: 31,
                        utf8_index: 49,
                        utf16_index: 49,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "add".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 4,
                        utf8_index: 80,
                        utf16_index: 78,
                    },
                    end: Position {
                        line: 5,
                        column: 7,
                        utf8_index: 83,
                        utf16_index: 81,
                    },
                },
            },
            Token {
                token_type: TokenType::Equal,
                content: "=".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 8,
                        utf8_index: 84,
                        utf16_index: 82,
                    },
                    end: Position {
                        line: 5,
                        column: 9,
                        utf8_index: 85,
                        utf16_index: 83,
                    },
                },
            },
            Token {
                token_type: TokenType::Lambda,
                content: "\\".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 10,
                        utf8_index: 86,
                        utf16_index: 84,
                    },
                    end: Position {
                        line: 5,
                        column: 11,
                        utf8_index: 87,
                        utf16_index: 85,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "n".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 11,
                        utf8_index: 87,
                        utf16_index: 85,
                    },
                    end: Position {
                        line: 5,
                        column: 12,
                        utf8_index: 88,
                        utf16_index: 86,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "m".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 13,
                        utf8_index: 89,
                        utf16_index: 87,
                    },
                    end: Position {
                        line: 5,
                        column: 14,
                        utf8_index: 90,
                        utf16_index: 88,
                    },
                },
            },
            Token {
                token_type: TokenType::Dot,
                content: ".".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 14,
                        utf8_index: 90,
                        utf16_index: 88,
                    },
                    end: Position {
                        line: 5,
                        column: 15,
                        utf8_index: 91,
                        utf16_index: 89,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "n".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 16,
                        utf8_index: 92,
                        utf16_index: 90,
                    },
                    end: Position {
                        line: 5,
                        column: 17,
                        utf8_index: 93,
                        utf16_index: 91,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "succ".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 18,
                        utf8_index: 94,
                        utf16_index: 92,
                    },
                    end: Position {
                        line: 5,
                        column: 22,
                        utf8_index: 98,
                        utf16_index: 96,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "m".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 23,
                        utf8_index: 99,
                        utf16_index: 97,
                    },
                    end: Position {
                        line: 5,
                        column: 24,
                        utf8_index: 100,
                        utf16_index: 98,
                    },
                },
            },
            Token {
                token_type: TokenType::In,
                content: "in".to_owned(),
                span: Span {
                    start: Position {
                        line: 6,
                        column: 1,
                        utf8_index: 101,
                        utf16_index: 99,
                    },
                    end: Position {
                        line: 6,
                        column: 3,
                        utf8_index: 103,
                        utf16_index: 101,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "add".to_owned(),
                span: Span {
                    start: Position {
                        line: 6,
                        column: 4,
                        utf8_index: 104,
                        utf16_index: 102,
                    },
                    end: Position {
                        line: 6,
                        column: 7,
                        utf8_index: 107,
                        utf16_index: 105,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "2".to_owned(),
                span: Span {
                    start: Position {
                        line: 6,
                        column: 8,
                        utf8_index: 108,
                        utf16_index: 106,
                    },
                    end: Position {
                        line: 6,
                        column: 9,
                        utf8_index: 109,
                        utf16_index: 107,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "3".to_owned(),
                span: Span {
                    start: Position {
                        line: 6,
                        column: 10,
                        utf8_index: 110,
                        utf16_index: 108,
                    },
                    end: Position {
                        line: 6,
                        column: 11,
                        utf8_index: 111,
                        utf16_index: 109,
                    },
                },
            },
        ]
    );
}
