//! Exporta um valor do cálculo Lambda.

/// `λa. λb. λc. a b c`
/// <=>
/// `λa. (λb. (λc. (a b) c))`
/// <=>
/// ```
/// Lambda {
///     parameter: "a",
///     body: Lambda {
///         parameter: "b",
///         body: Lambda {
///             parameter: "c",
///             body: Application {
///                 function: Application {
///                     function: Variable("a"),
///                     argument: Variable("b"),
///                 },
///                 argument: Variable("c"),
///             },
///         },
///     },
/// }
/// ```
///
/// `(λx. a x) (λy. y)`
/// <=>
/// ```
/// Application {
///     function: Lambda {
///         parameter: "x",
///         body: Application {
///             function: Variable("a"),
///             argument: Variable("x"),
///         },
///     },
///     argument: Lambda {
///         parameter: "y",
///         body: Variable("y"),
///     },
/// }
///
/// reduced = `a (λy. y)`
/// ```
#[derive(Debug, Clone)]
pub enum Value {
    /// `x  <=>     Variable("x")`
    Variable(String),

    /// `f x <=>    Application {
    ///                 function: Variable("f"),
    ///                 argument: Variable("x")
    ///             }`
    Application { function: Box<Value>, argument: Box<Value> },

    /// `λx. y <=>  Lambda {
    ///                 parameter: "x",
    ///                 body: Variable("y"),
    ///             }`
    Lambda { parameter: String, body: Box<Value> },
}

impl Value {
    /// Não lida com a captura de variáveis.
    ///
    /// # Captura de variáveis.
    ///
    /// ```
    /// λa. (λx. λa. x a) a
    /// ```
    /// <=> substituir `x` por `a`
    /// ```
    /// λa. (λa. a a)
    /// ```
    ///
    /// Aqui ocorreu uma captura de variáveis, um erro, pois os dois `a` se
    /// referem a coisas diferentes, mas após a substituição, os dois não são
    /// distinguidos e aparentam ser ambos referências ao parâmetro mais
    /// interno.
    ///
    /// Solução mais básica? Trocar nome do parâmetro.
    /// ```
    /// λa. (λb. a b)
    /// ```
    pub fn replace(&mut self, target: &str, value: &Self) {
        match self {
            Value::Variable(variable) => {
                if variable == target {
                    *self = value.clone();
                }
            }

            Value::Application { function, argument } => {
                function.replace(target, value);
                argument.replace(target, value);
            }

            Value::Lambda { parameter, body } => {
                if parameter != target {
                    body.replace(target, value);
                }
            }
        }
    }
}
