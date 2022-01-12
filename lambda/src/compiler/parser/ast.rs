use crate::compiler::position::Span;

// <program> ::=
//      | <let> <bindings> <in> <expr>
//      | <expr>
//
// // repete
// <bindings> ::= <var> = <expr> ;
//
// <expr> ::=
//      | <var>
//      | <expr> <expr>
//      | \<var>. <expr>
//      | (<expr>)

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    Variable(Symbol),
    Number(u64),
    Application { function: Box<Expr>, argument: Box<Expr> },
    Lambda { parameter: Symbol, body: Box<Expr> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub main_expression: Expr,
    pub bindings: Vec<Binding>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Binding {
    pub name: Symbol,
    pub expression: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    ///
    /// - `content`: palavra do código, mas que não é necessariamente código em si
    pub content: String,
    ///
    /// - `span`: localização dessa palavra no código
    pub span: Span,
}
