//! Exporta um valor do cálculo Lambda.

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;

/// Representação recursiva de um termo Lambda.
///
/// `λa. λb. λc. a b c`
/// <=>
/// `λa. (λb. (λc. (a b) c))`
/// <=>
/// ```ignore
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
/// ```ignore
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
    /// Retorna a codificação de church do dado número natural.
    pub fn church_numeral(number: u64) -> Self {
        let mut body = Value::Variable(String::from("x"));

        for _ in 0..number {
            body = Value::Application {
                function: Box::new(Value::Variable(String::from("f"))),
                argument: Box::new(body),
            };
        }

        Value::Lambda {
            parameter: String::from("f"),
            body: Box::new(Value::Lambda {
                parameter: String::from("x"),
                body: Box::new(body),
            }),
        }
    }

    pub fn beta_equiv(&self, other: &Value) -> bool {
        let mut self_indices = ParamIndices::default();
        let mut other_indices = ParamIndices::default();
        self.beta_equiv_with(other, &mut self_indices, &mut other_indices)
    }

    fn beta_equiv_with<'this, 'other>(
        &'this self,
        other: &'other Value,
        self_indices: &mut ParamIndices<'this>,
        other_indices: &mut ParamIndices<'other>,
    ) -> bool {
        match (self, other) {
            (Value::Variable(self_var), Value::Variable(other_var)) => {
                match (
                    self_indices.get(self_var.as_str()),
                    other_indices.get(other_var.as_str()),
                ) {
                    (Some(self_index), Some(other_index)) => {
                        self_index == other_index
                    }
                    (Some(_), None) | (None, Some(_)) => false,
                    (None, None) => self_var == other_var,
                }
            }

            (
                Value::Application { function: self_func, argument: self_arg },
                Value::Application {
                    function: other_func,
                    argument: other_arg,
                },
            ) => {
                self_func.beta_equiv_with(
                    other_func,
                    self_indices,
                    other_indices,
                ) && self_arg.beta_equiv_with(
                    other_arg,
                    self_indices,
                    other_indices,
                )
            }

            (
                Value::Lambda { parameter: self_param, body: self_body },
                Value::Lambda { parameter: other_param, body: other_body },
            ) => {
                let self_old_index = self_indices.push(self_param);
                let other_old_index = other_indices.push(other_param);
                let body_is_beta_equiv = self_body.beta_equiv_with(
                    other_body,
                    self_indices,
                    other_indices,
                );
                self_indices.pop(self_param, self_old_index);
                other_indices.pop(other_param, other_old_index);
                body_is_beta_equiv
            }

            _ => false,
        }
    }

    /// Substitui todas as ocorrências da variável `target_var` pelo valor `new_value` dentro de `self`.
    /// Lida com a captura de variáveis.
    ///
    /// # Captura de variáveis.
    ///
    /// ```ignore
    /// λa. (λx. λa. x a) a
    /// ```
    /// <=> substituir `x` por `a`
    /// ```ignore
    /// λa. (λa. a a)
    /// ```
    ///
    /// Aqui ocorreu uma captura de variáveis, um erro, pois os dois `a` se
    /// referem a coisas diferentes, mas após a substituição, os dois não são
    /// distinguidos e aparentam ser ambos referências ao parâmetro mais
    /// interno.
    ///
    /// Solução mais básica? Trocar nome do parâmetro.
    /// ```ignore
    /// λa. (λa_. a a_)
    /// ```
    ///
    /// No entanto, é preciso cuidar o seguinte:
    /// ```ignore
    /// λa_. λa. (λx. λa. x a a_) a
    /// ```
    /// <=> substituir `x` por `a`
    /// ```ignore
    /// λa_. λa. (λa_. a a_ a_) a
    /// ```
    ///
    /// Mas a resposta correta deve ser:
    /// ```ignore
    /// λa_. λa. (λa__. a a__ a_) a
    /// ```
    ///
    /// Solução mais básica? Aumentar o nome da variável com `_` até não haver
    /// variáveis livres.
    pub fn replace(&mut self, target_var: &str, new_value: &Self) {
        let unbounded_vars = new_value.unbounded_vars();
        self.replace_with(target_var, new_value, &unbounded_vars);
    }

    fn replace_with(
        &mut self,
        target_var: &str,
        new_value: &Self,
        new_value_unbounded: &HashSet<&str>,
    ) {
        match self {
            Value::Variable(variable) => {
                if variable == target_var {
                    *self = new_value.clone();
                }
            }

            Value::Application { function, argument } => {
                function.replace_with(
                    target_var,
                    new_value,
                    new_value_unbounded,
                );
                argument.replace_with(
                    target_var,
                    new_value,
                    new_value_unbounded,
                );
            }

            Value::Lambda { parameter, body } => {
                if parameter != target_var {
                    if new_value_unbounded.contains(parameter.as_str()) {
                        let body_unbounded = body.unbounded_vars();
                        let mut renamed_var = format!("{}_", parameter);
                        while new_value_unbounded.contains(renamed_var.as_str())
                            || body_unbounded.contains(renamed_var.as_str())
                        {
                            renamed_var.push('_');
                        }
                        body.replace(
                            parameter.as_str(),
                            &Value::Variable(renamed_var.clone()),
                        );
                        *parameter = renamed_var;
                    }
                    body.replace_with(
                        target_var,
                        new_value,
                        new_value_unbounded,
                    );
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

#[derive(Debug, Clone, Default)]
struct ParamIndices<'value> {
    param_map: HashMap<&'value str, u64>,
    count: u64,
}

impl<'value> ParamIndices<'value> {
    #[must_use]
    pub fn push(&mut self, param: &'value str) -> Option<u64> {
        let old_index = self.param_map.insert(param, self.count);
        self.count += 1;
        old_index
    }

    pub fn pop(&mut self, param: &'value str, old_index: Option<u64>) {
        match old_index {
            Some(index) => {
                self.param_map.insert(param, index);
            }
            None => {
                self.param_map.remove(param);
            }
        }
        self.count -= 1;
    }

    pub fn get(&self, param: &'value str) -> Option<u64> {
        self.param_map.get(param).copied().map(|index| self.count - index)
    }
}
