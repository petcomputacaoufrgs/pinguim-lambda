use super::{
    super::{
        error::Diagnostics,
        position::{Position, Span},
    },
    generate_tokens,
    token::{Token, TokenType},
};

/* 
    - Código fonte de tamanho 0
    - Código simples sem keyword
    - Bloco de let sozinho (sem in)
    - Bloco de let com bloco de in
    - Teste com numerais de church
    - Teste com números naturais
    - Teste com comentários bem e mal formatados
    - Teste com caracteres inválidos (para os quais não existe TokenType)
    - Teste com múltiplos erros para ver o diagnostics
*/

#[test]
fn empty_src() {
    let mut diagnostics = Diagnostics::new();
    let empty_code = "";
    let tokens = generate_tokens(empty_code, &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(tokens, Vec::new());
}

//    - Código simples sem keyword  \a b. a
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

// - Bloco de let sozinho (sem in) let\n    true = \a b. a ;
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