//! Exporta um valor do cálculo Lambda.

#[cfg(test)]
mod test;

use std::collections::HashSet;
use std::mem;

/// Representação recursiva de um termo Lambda.
///
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    /// Uma variável.
    ///
    /// `x  <=>     Variable("x")`
    Variable(String),

    /// Uma aplicação de um argumento em uma função.
    ///
    /// `f x <=>    Application {
    ///                 function: Variable("f"),
    ///                 argument: Variable("x")
    ///             }`
    Application { function: Box<Value>, argument: Box<Value> },

    /// Uma abstração Lambda com parâmetro e corpo.
    ///
    /// `λx. y <=>  Lambda {
    ///                 parameter: "x",
    ///                 body: Variable("y"),
    ///             }`
    Lambda { parameter: String, body: Box<Value> },
}

impl Value {
    /// Substitui todas as ocorrências da variável `target_var` pelo valor `new_value` dentro de `self`.
    /// Lida com a captura de variáveis.
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
    /// λa. (λa_. a a_)
    /// ```
    pub fn replace(&mut self, target_var: &str, new_value: &Self) {
        let unbounded_vars = new_value.unbounded_vars();
        self.replace_with(target_var, new_value, &unbounded_vars);
    }

    fn replace_with(
        &mut self,
        target_var: &str,
        new_value: &Self,
        unbounded_vars: &HashSet<&str>,
    ) {
        match self {
            Value::Variable(variable) => {
                if variable == target_var {
                    *self = new_value.clone();
                }
            }

            Value::Application { function, argument } => {
                function.replace(target_var, new_value);
                argument.replace(target_var, new_value);
            }

            Value::Lambda { parameter, body } => {
                if parameter != target_var {
                    if unbounded_vars.contains(parameter.as_str()) {
                        let renamed_var =
                            Value::Variable(format!("{}_", parameter));
                        body.replace(parameter.as_str(), &renamed_var);
                        *parameter += "_";
                    }
                    body.replace(target_var, new_value);
                }
            }
        }
    }

    /// Faz a redução de um único redex, mais externo, mais à esquerda. Retorna se tal redex foi encontrado.
    pub fn reduce_one(&mut self) -> bool {
        match self {
            Value::Variable(_) => false,

            Value::Application { function, argument } => {
                if let Value::Lambda { parameter, body } = function.as_mut() {
                    body.replace(parameter, argument);
                    *self = mem::replace(
                        body.as_mut(),
                        Value::Variable(String::new()),
                    );
                    true
                } else {
                    function.reduce_one() || argument.reduce_one()
                }
            }

            Value::Lambda { parameter: _, body } => body.reduce_one(),
        }
    }

    /// Reduz o termo até a sua forma normal, se existir. Se não existir, entra em loop infinito.
    pub fn reduce(&mut self) {
        while self.reduce_one() {}
    }

    /// Retorna o conjunto das varíaveis não ligadas nesse termo.
    pub fn unbounded_vars(&self) -> HashSet<&str> {
        let mut unbounded_set = HashSet::new();
        let mut bounded_set = HashSet::new();

        self.unbounded_vars_at(&mut unbounded_set, &mut bounded_set);
        unbounded_set
    }

    fn unbounded_vars_at<'value>(
        &'value self,
        unbounded_set: &mut HashSet<&'value str>,
        bounded_set: &mut HashSet<&'value str>,
    ) {
        match self {
            Value::Variable(variable) => {
                if !bounded_set.contains(variable.as_str()) {
                    unbounded_set.insert(variable);
                }
            }
            Value::Application { function, argument } => {
                function.unbounded_vars_at(unbounded_set, bounded_set);
                argument.unbounded_vars_at(unbounded_set, bounded_set);
            }
            Value::Lambda { parameter, body } => {
                let was_inserted = bounded_set.insert(parameter);
                body.unbounded_vars_at(unbounded_set, bounded_set);
                if was_inserted {
                    bounded_set.remove(parameter.as_str());
                }
            }
        }
    }
}
