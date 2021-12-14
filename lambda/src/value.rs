//! Exporta um valor do cálculo Lambda.

#[cfg(test)]
mod test;

use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
use std::ops::{Deref, DerefMut};

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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    Application { function: NestedValue, argument: NestedValue },

    /// Uma abstração Lambda com parâmetro e corpo.
    ///
    /// `λx. y <=>  Lambda {
    ///                 parameter: "x",
    ///                 body: Variable("y"),
    ///             }`
    Lambda { parameter: String, body: NestedValue },
}

impl Value {
    fn dummy() -> Self {
        Value::Variable(String::new())
    }

    /// Retorna a codificação de church do dado número natural.
    pub fn church_numeral(number: u64) -> Self {
        let mut body = Value::Variable(String::from("x"));

        for _ in 0..number {
            body = Value::Application {
                function: NestedValue::new(Value::Variable(String::from("f"))),
                argument: NestedValue::new(body),
            };
        }

        Value::Lambda {
            parameter: String::from("f"),
            body: NestedValue::new(Value::Lambda {
                parameter: String::from("x"),
                body: NestedValue::new(body),
            }),
        }
    }

    /// Testa se dois termos são beta-equivalentes.
    pub fn beta_equiv(&self, other: &Value) -> bool {
        let mut self_indices = ParamIndices::default();
        let mut other_indices = ParamIndices::default();

        /// Uma operação/passo para computar a beta-equivalência de dois termos.
        enum Operation<'this, 'other> {
            /// Testa se dois valores são beta-equivalentes.
            Compare(&'this Value, &'other Value),
            /// Remove um parâmetro do mapeamento de indices de self. Restaurando um possível antigo valor.
            PopSelfIndex(&'this str, Option<u64>),
            /// Remove um parâmetro do mapeamento de indices de other. Restaurando um possível antigo valor.
            PopOtherIndex(&'other str, Option<u64>),
        }

        let mut equals = true;
        let mut operation_stack = vec![Operation::Compare(self, other)];

        while let Some(operation) = operation_stack.pop().filter(|_| equals) {
            match operation {
                Operation::Compare(self_value, other_value) => {
                    match (self_value, other_value) {
                        (
                            Value::Variable(self_var),
                            Value::Variable(other_var),
                        ) => {
                            equals = match (
                                self_indices.get(self_var.as_str()),
                                other_indices.get(other_var.as_str()),
                            ) {
                                (Some(self_index), Some(other_index)) => {
                                    self_index == other_index
                                }
                                (Some(_), None) | (None, Some(_)) => false,
                                (None, None) => self_var == other_var,
                            };
                        }

                        (
                            Value::Application {
                                function: self_func,
                                argument: self_arg,
                            },
                            Value::Application {
                                function: other_func,
                                argument: other_arg,
                            },
                        ) => {
                            operation_stack.push(Operation::Compare(
                                self_func, other_func,
                            ));
                            operation_stack
                                .push(Operation::Compare(self_arg, other_arg));
                        }

                        (
                            Value::Lambda {
                                parameter: self_param,
                                body: self_body,
                            },
                            Value::Lambda {
                                parameter: other_param,
                                body: other_body,
                            },
                        ) => {
                            let self_old_index = self_indices.push(self_param);
                            let other_old_index =
                                other_indices.push(other_param);

                            operation_stack.push(Operation::PopSelfIndex(
                                self_param,
                                self_old_index,
                            ));
                            operation_stack.push(Operation::PopOtherIndex(
                                other_param,
                                other_old_index,
                            ));

                            operation_stack.push(Operation::Compare(
                                self_body, other_body,
                            ));
                        }

                        _ => equals = false,
                    }
                }

                Operation::PopOtherIndex(param, old_index) => {
                    other_indices.pop(param, old_index);
                }
                Operation::PopSelfIndex(param, old_index) => {
                    self_indices.pop(param, old_index);
                }
            }
        }

        equals
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
        enum Replacement<'this, 'var, 'new_value> {
            Main(&'var str, &'new_value Value, HashSet<&'new_value str>),
            Rename { old_parameter: String, new_parameter: &'this str },
        }

        impl<'this, 'var, 'new_value> Replacement<'this, 'var, 'new_value> {
            fn is_unbound_var(&self, var_name: &str) -> bool {
                match self {
                    Replacement::Main(_, _, unbound_set) => {
                        unbound_set.contains(var_name)
                    }
                    Replacement::Rename { new_parameter, .. } => {
                        *new_parameter == var_name
                    }
                }
            }

            fn target_var(&self) -> &str {
                match self {
                    Replacement::Main(target_var, _, _) => target_var,
                    Replacement::Rename { old_parameter, .. } => old_parameter,
                }
            }

            fn clone_new_value(&self) -> Value {
                match self {
                    Replacement::Main(_, new_value, _) => (*new_value).clone(),
                    Replacement::Rename { new_parameter, .. } => {
                        Value::Variable((*new_parameter).to_owned())
                    }
                }
            }
        }

        enum Operation<'this> {
            Replace(&'this mut Value),
            DropReplacement,
        }

        let mut replacements =
            vec![Replacement::new_borrowed(target_var, new_value)];
        let mut operation_stack = vec![Operation::Replace(self)];

        while let Some(operation) = operation_stack.pop() {
            match operation {
                Operation::Replace(value) => match value {
                    Value::Variable(variable) => {
                        if variable == &replacement.target_var {
                            *value = replacement.new_value.clone().into_owned();
                        }
                    }

                    Value::Application { function, argument } => {
                        operation_stack.push(Operation::Replace(function));
                        operation_stack.push(Operation::Replace(argument));
                    }

                    Value::Lambda { parameter, body } => {
                        if parameter != &replacement.target_var {
                            operation_stack.push(Operation::Replace(body));
                            if replacement
                                .unbound_vars
                                .contains(parameter.as_str())
                            {
                                let body_unbound =
                                    body.unbound_vars().collect::<HashSet<_>>();
                                let mut renamed_var = format!("{}_", parameter);
                                while replacement
                                    .unbound_vars
                                    .contains(renamed_var.as_str())
                                    || body_unbound
                                        .contains(renamed_var.as_str())
                                {
                                    renamed_var.push('_');
                                }
                                let old_parameter = mem::replace(
                                    parameter,
                                    renamed_var.clone(),
                                );
                                replacement_stack.push(mem::replace(
                                    &mut replacement,
                                    Replacement::new_owned(
                                        old_parameter,
                                        Value::Variable(renamed_var),
                                    ),
                                ));
                                operation_stack
                                    .push(Operation::DropReplacement);
                                operation_stack.push(Operation::Replace(body));
                            }
                        }
                    }
                },
                Operation::DropReplacement => {
                    replacement =
                        replacement_stack.pop().expect("replacement pop");
                }
            }
        }

        todo!();

        /*
        let unboud_vars = new_value.unbound_vars().collect();
        self.replace_with(target_var, new_value, &unboud_vars)
        */
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
                        let body_unbound =
                            body.unbound_vars().collect::<HashSet<_>>();
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
        let mut candidate_stack = vec![self];
        let mut redex_found = false;

        while let Some(candidate) =
            candidate_stack.pop().filter(|_| !redex_found)
        {
            if let Value::Application { function, argument } = candidate {
                if let Value::Lambda { parameter, body } =
                    function.as_mut_value()
                {
                    body.replace(parameter, argument);
                    *candidate =
                        mem::replace(body.as_mut_value(), Value::dummy());
                    redex_found = true;
                }
            }

            if !redex_found {
                match candidate {
                    Value::Variable(_) => (),
                    Value::Application { function, argument } => {
                        candidate_stack.push(argument);
                        candidate_stack.push(function);
                    }
                    Value::Lambda { parameter: _, body } => {
                        candidate_stack.push(body);
                    }
                }
            }
        }

        redex_found
    }

    /// Cria um iterador sobre as variáveis não-ligadas neste termo. Variáveis podem aparecer mais de uma vez.
    pub fn unbound_vars(&self) -> UnboundVars {
        UnboundVars {
            operation_stack: vec![UnboundVarsOper::Visit(self)],
            bound_set: HashSet::new(),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        /// Uma operação auxiliar de clonagem.
        enum Operation<'value> {
            /// Inicia a clonagem de um termo qualquer.
            Clone(&'value Value),

            /// Finaliza a clonagem de um termo lambda.
            MakeLambda(String),

            /// Finaliza a clonagem de um termo aplicação.
            MakeApplication,
        }

        let mut operation_stack = vec![Operation::Clone(self)];
        let mut output_stack = Vec::new();

        while let Some(operation) = operation_stack.pop() {
            match operation {
                Operation::Clone(value) => match value {
                    Value::Variable(variable) => {
                        output_stack.push(Value::Variable(variable.clone()));
                    }
                    Value::Application { function, argument } => {
                        operation_stack.push(Operation::MakeApplication);
                        operation_stack.push(Operation::Clone(argument));
                        operation_stack.push(Operation::Clone(function));
                    }
                    Value::Lambda { parameter, body } => {
                        operation_stack
                            .push(Operation::MakeLambda(parameter.clone()));
                        operation_stack.push(Operation::Clone(body));
                    }
                },

                Operation::MakeLambda(parameter) => {
                    let body = output_stack.pop().expect("clone value body");
                    output_stack.push(Value::Lambda {
                        parameter,
                        body: NestedValue::new(body),
                    });
                }

                Operation::MakeApplication => {
                    let argument =
                        output_stack.pop().expect("clone value argument");
                    let function =
                        output_stack.pop().expect("clone value function");

                    output_stack.push(Value::Application {
                        function: NestedValue::new(function),
                        argument: NestedValue::new(argument),
                    });
                }
            }
        }
        output_stack.pop().expect("clone value")
    }
}

/// Iterador que produz nomes de variáveis não-ligadas em um termo lambda. Nome de variáveis podem repetir.
#[derive(Debug, Clone)]
pub struct UnboundVars<'value> {
    /// Pilha de operações/passos para coletar variáveis não-ligadas.
    operation_stack: Vec<UnboundVarsOper<'value>>,
    /// Rastreio de variáveis já ligadas.
    bound_set: HashSet<&'value str>,
}

impl<'value> Iterator for UnboundVars<'value> {
    type Item = &'value str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(operation) = self.operation_stack.pop() {
            match operation {
                UnboundVarsOper::Visit(value) => match value {
                    Value::Variable(variable) => {
                        if !self.bound_set.contains(variable.as_str()) {
                            return Some(variable);
                        }
                    }
                    Value::Application { function, argument } => {
                        self.operation_stack
                            .push(UnboundVarsOper::Visit(function));
                        self.operation_stack
                            .push(UnboundVarsOper::Visit(argument));
                    }
                    Value::Lambda { parameter, body } => {
                        let was_inserted = self.bound_set.insert(parameter);
                        if was_inserted {
                            self.operation_stack
                                .push(UnboundVarsOper::RemoveBound(parameter));
                        }
                        self.operation_stack.push(UnboundVarsOper::Visit(body));
                    }
                },

                UnboundVarsOper::RemoveBound(parameter) => {
                    self.bound_set.remove(parameter);
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
/// Um passo para calcular o conjunto das variáveis não-ligadas. Uso interno do iterador de variáveis não-ligadas.
enum UnboundVarsOper<'value> {
    /// Visita um termo para coletar suas variáveis não-ligadas.
    Visit(&'value Value),
    /// Remove uma variável do conjunto das variáveis ligadas.
    RemoveBound(&'value str),
}

/// Um termo aninhado de cálculo lambda com ponteiro indireto para o termo contido.
/// Implementação de drop não é recursiva.
/// Derreferencia para o termo contido automáticamente.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NestedValue {
    /// O termo contido em uma expressão aninhada.
    inner: Box<Value>,
}

impl NestedValue {
    /// Cria um termo aninhado a partir de um termo.
    pub fn new(value: Value) -> Self {
        Self { inner: Box::new(value) }
    }

    /// Explícitamente obtém uma referência para o termo contido.
    pub fn as_value(&self) -> &Value {
        &self.inner
    }

    /// Explícitamente obtém uma referência mutável para o termo contido.
    pub fn as_mut_value(&mut self) -> &mut Value {
        &mut self.inner
    }

    /// Converte o termo aninhado para termo contido.
    pub fn into_value(mut self) -> Value {
        self.take_value()
    }

    /// Toma o termo contido, substituindo-o por um termo qualquer.
    /// Não chamar diretamente.
    fn take_value(&mut self) -> Value {
        mem::replace(&mut self.inner, Value::dummy())
    }
}

impl Deref for NestedValue {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        self.as_value()
    }
}

impl DerefMut for NestedValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_value()
    }
}

impl Drop for NestedValue {
    fn drop(&mut self) {
        let mut cur_value = Some(self.take_value());
        let mut drop_stack = Vec::new();

        while let Some(value) = cur_value.take() {
            match value {
                Value::Variable(_) => (),
                Value::Application { function, argument } => {
                    drop_stack.push(function.into_value());
                    drop_stack.push(argument.into_value());
                }
                Value::Lambda { parameter: _, body } => {
                    drop_stack.push(body.into_value());
                }
            }

            cur_value = drop_stack.pop();
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
