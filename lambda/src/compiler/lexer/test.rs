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


