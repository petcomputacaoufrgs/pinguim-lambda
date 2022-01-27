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
/// ```text
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
/// ```text
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

    /// Testa se dois termos são beta-equivalentes.
    pub fn beta_equiv(&self, other: &Value) -> bool {
        let mut self_indices = ParamIndices::default();
        let mut other_indices = ParamIndices::default();
        self.beta_equiv_with(other, &mut self_indices, &mut other_indices)
    }

    /// Detalhe de implementação do teste de equivalência.
    /// Testa a beta-equivalência recursivamente, usando estruturas auxiliares já inicializadas.
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
    /// ```text
    /// λa. (λx. λa. x a) a
    /// ```
    /// <=> substituir `x` por `a`
    /// ```text
    /// λa. (λa. a a)
    /// ```
    ///
    /// Aqui ocorreu uma captura de variáveis, um erro, pois os dois `a` se
    /// referem a coisas diferentes, mas após a substituição, os dois não são
    /// distinguidos e aparentam ser ambos referências ao parâmetro mais
    /// interno.
    ///
    /// Solução mais básica? Trocar nome do parâmetro.
    /// ```text
    /// λa. (λa_. a a_)
    /// ```
    ///
    /// No entanto, é preciso cuidar o seguinte:
    /// ```text
    /// λa_. λa. (λx. λa. x a a_) a
    /// ```
    /// <=> substituir `x` por `a`
    /// ```text
    /// λa_. λa. (λa_. a a_ a_) a
    /// ```
    ///
    /// Mas a resposta correta deve ser:
    /// ```text
    /// λa_. λa. (λa__. a a__ a_) a
    /// ```
    ///
    /// Solução mais básica? Aumentar o nome da variável com `_` até não haver
    /// variáveis livres.
    pub fn replace(&mut self, target_var: &str, new_value: &Self) {
        let unbound_vars = new_value.unbound_vars();
        self.replace_with(target_var, new_value, &unbound_vars);
    }

    /// Detalhe de implementação da substituição de variáveis.
    /// Realiza a substituição recursivamente, utilizando estruturas auxiliares já inicializadas.
    fn replace_with(
        &mut self,
        target_var: &str,
        new_value: &Self,
        new_value_unbound: &HashSet<&str>,
    ) {
        match self {
            Value::Variable(variable) => {
                if variable == target_var {
                    *self = new_value.clone();
                }
            }

            Value::Application { function, argument } => {
                function.replace_with(target_var, new_value, new_value_unbound);
                argument.replace_with(target_var, new_value, new_value_unbound);
            }

            Value::Lambda { parameter, body } => {
                if parameter != target_var {
                    if new_value_unbound.contains(parameter.as_str()) {
                        let body_unbound = body.unbound_vars();
                        let mut renamed_var = format!("{}_", parameter);
                        while new_value_unbound.contains(renamed_var.as_str())
                            || body_unbound.contains(renamed_var.as_str())
                        {
                            renamed_var.push('_');
                        }
                        body.replace(
                            parameter.as_str(),
                            &Value::Variable(renamed_var.clone()),
                        );
                        *parameter = renamed_var;
                    }
                    body.replace_with(target_var, new_value, new_value_unbound);
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

    /// Retorna o conjunto das varíaveis não ligadas nesse termo.
    pub fn unbound_vars(&self) -> HashSet<&str> {
        let mut unbound_set = HashSet::new();
        let mut bound_set = HashSet::new();

        self.unbound_vars_at(&mut unbound_set, &mut bound_set);
        unbound_set
    }

    /// Detalhe de implementação da construção do conjunto de variáveis não ligadas.
    /// Realiza a construção recursivamente, utilizando estruturas auxiliares já inicializadas.
    fn unbound_vars_at<'value>(
        &'value self,
        unbound_set: &mut HashSet<&'value str>,
        bound_set: &mut HashSet<&'value str>,
    ) {
        match self {
            Value::Variable(variable) => {
                if !bound_set.contains(variable.as_str()) {
                    unbound_set.insert(variable);
                }
            }
            Value::Application { function, argument } => {
                function.unbound_vars_at(unbound_set, bound_set);
                argument.unbound_vars_at(unbound_set, bound_set);
            }
            Value::Lambda { parameter, body } => {
                let was_inserted = bound_set.insert(parameter);
                body.unbound_vars_at(unbound_set, bound_set);
                if was_inserted {
                    bound_set.remove(parameter.as_str());
                }
            }
        }
    }
}

/// Mapeamento de nomes de parâmetros para indices.
#[derive(Debug, Clone, Default)]
struct ParamIndices<'value> {
    /// Associação entre nomes de parâmetros e indices.
    param_map: HashMap<&'value str, u64>,
    /// Quantidade de parâmetros.
    param_count: u64,
}

impl<'value> ParamIndices<'value> {
    /// Mapeia o nome do parâmetro para o último indice dessa coleção.
    ///
    /// O método retorna um possível mapeamento antigo para esse nome de parâmetro.
    /// Deve-se passar esse antigo mapeamento para o método [`ParamIndices::pop`], ao remover o mapeamento atual.
    #[must_use]
    pub fn push(&mut self, param: &'value str) -> Option<u64> {
        let old_index = self.param_map.insert(param, self.param_count);
        self.param_count += 1;
        old_index
    }

    /// Remove o último mapeamento, restaurando um possível mapeamento antigo para esse nome de parâmetro.
    ///
    /// O método não checa se o nome de parâmetro passado de fato está no último mapeamento,
    /// é responsabilidade de quem chama o método passar o parâmetro correto.
    ///
    /// O antigo mapeamento deve ser o retornado pelo método [`ParamIndices::push`] que criou um mapeamento sendo removido.
    pub fn pop(&mut self, param: &'value str, old_index: Option<u64>) {
        match old_index {
            Some(index) => {
                self.param_map.insert(param, index);
            }
            None => {
                self.param_map.remove(param);
            }
        }
        self.param_count -= 1;
    }

    /// Obtém o indice mapeado para aquele nome de parâmetro, se estiver mapeado.
    /// O último parâmetro mapeado terá índice 1, o segundo índice 2, etc...
    pub fn get(&self, param: &'value str) -> Option<u64> {
        self.param_map.get(param).copied().map(|index| self.param_count - index)
    }
}
