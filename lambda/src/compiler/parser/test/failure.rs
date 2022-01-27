use crate::compiler::lexer::generate_tokens;
use crate::compiler::parser::{ast, parse};
use crate::compiler::{
    error::Diagnostics,
    position::{Position, Span},
};

/*
    - erro de lambda sem parametro
    - erro de lambda sem corpo
    - erro de código vazio
    - erro de parenteses aberto que não foi fechado
        - numa expressão delimitada por ";" ou "in"
        - numa expressão no final do código
    - erro de parenteses fechando sem parenteses aberto associado
    - erro de faltando token equal no binding
    - erro de faltando "in" no código
    - erro de faltando "let" no início do código
    - erro de parênteses nos parâmetros do lambda
    - erro de lambda sem ponto
        - só vai parar no final do arquivo! (melhorar isso - talvez usar o ExprEnd)
*/
