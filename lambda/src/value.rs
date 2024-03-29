//! Exporta um valor do cálculo Lambda.

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
use std::ops::{Deref, DerefMut};

/// Representação recursiva de um termo Lambda. Equivalente a:
/// ```haskell
/// data Value =
///       Variable String
///     | Application Value Value
///     | Lambda String Value
/// ```
///
/// # Exemplos
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
#[derive(Debug, Eq, PartialOrd, Ord, Hash)]
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
    ///
    /// # Algoritmo Recursivo
    ///
    /// ```haskell
    /// churchNum :: Int -> Value
    /// churchNum n =
    ///   let body 0 = Variable "x"
    ///       body m = Application (Variable "f") (body (m - 1))
    ///   in Lambda "f" (Lambda "x" (body n))
    /// ```
    pub fn church_numeral(number: u32) -> Self {
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

    /// Se esse termo for um numeral de church, este método converte o termo para um inteiro do Rust.
    /// Se não for, o método retorna `None`.
    ///
    /// # Algoritmo Recursivo
    ///
    /// ```haskell
    /// churchNumToInt :: Value -> Maybe Int
    ///
    /// churchNumToInt (Lambda pf (Lambda px b)) =
    ///   let checkBody (Application (Variable f) a) =
    ///         if f == pf
    ///           then case checkBody a of
    ///             Just n -> Just (n + 1)
    ///             Nothing -> Nothing
    ///           else Nothing
    ///       checkBody (Variable s) =
    ///         if s == px
    ///           then Just 0
    ///           else Nothing
    ///       checkBody _ = Nothing
    ///   in checkBody b
    ///
    /// churchNumToInt _ = Nothing
    /// ```
    pub fn church_numeral_to_int(&self) -> Option<u32> {
        let (param_f, param_x, mut body) = match self {
            Value::Lambda { parameter: param_f, body } => match body.as_value()
            {
                Value::Lambda { parameter: param_x, body } => {
                    Some((param_f, param_x, body.as_value()))
                }
                _ => None,
            },
            _ => None,
        }?;

        let mut converted_int = 0;
        loop {
            match body {
                Value::Variable(variable) if variable == param_x => break,
                Value::Application { function, argument } => {
                    match function.as_value() {
                        Value::Variable(variable) if variable == param_f => {
                            converted_int += 1;
                            body = argument;
                        }
                        _ => None?,
                    }
                }
                _ => None?,
            }
        }

        Some(converted_int)
    }

    /// Testa se dois termos são beta-equivalentes.
    ///
    /// # Algoritmo Recursivo
    ///
    /// ```haskell
    /// betaEquiv :: Value -> Value -> Bool
    /// betaEquiv v1 v2 =
    ///   let deBruijnIndex s [] = Nothing
    ///       deBruijnIndex s (x : xs) =
    ///         if x == s
    ///           then Just 0
    ///           else case deBruijnIndex s xs of
    ///             Just i -> Just (i + 1)
    ///             Nothing -> Nothing
    ///
    ///       betaEquivWith (Variable s1) (Variable s2) ps1 ps2 =
    ///         case (deBruijnIndex s1 ps1, deBruijnIndex s2 ps2) of
    ///           (Just i, Just j) -> i == j
    ///           (Nothing, Nothing) -> s1 == s2
    ///           _ -> False
    ///
    ///       betaEquivWith (Application f1 a1) (Application f2 a2) ps1 ps2 =
    ///         betaEquivWith f1 f2 ps1 ps2 && betaEquivWith a1 a2 ps1 ps2
    ///
    ///       betaEquivWith (Lambda p1 b1) (Lambda p2 b2) ps1 ps2 =
    ///         betaEquivWith b1 b2 (p1 : ps1) (p2 : ps2)
    ///
    ///       betaEquivWith _ _ ps1 ps2 = False
    ///
    ///   in betaEquivWith v1 v2 [] []
    /// ```
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
                            // .pop() retorna elementos na ordem inversa do .push()
                            operation_stack
                                .push(Operation::Compare(self_arg, other_arg));
                            operation_stack.push(Operation::Compare(
                                self_func, other_func,
                            ));
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
    /// # Exemplo
    ///
    /// ```text
    /// substituir (x) por (f y) em (x (\h. h x) (\x. x z))
    /// =>
    /// (f y (\h. h (f y)) (\x. x z))
    /// ```
    ///
    /// # Algoritmo Recursivo
    ///
    /// ```haskell
    /// replace :: Value -> String -> Value -> Value
    ///
    /// replace (Variable s) t v =
    ///   if s == t
    ///     then v
    ///     else Variable s
    ///
    /// replace (Application f a) t v =
    ///   Application (replace f t v) (replace a t v)
    ///
    /// replace (Lambda p b) t v =
    ///   if p == t
    ///     then Lambda p b
    ///     else if elem p (unboundVars v)
    ///       then
    ///         let rename s =
    ///               if elem s (unboundVars b) || elem s (unboundVars v)
    ///                 then rename (s ++ "_")
    ///                 else s
    ///             p' = rename p
    ///             b' = replace b p (Variable p')
    ///         in Lambda p' (replace b' t v)
    ///       else Lambda p (replace b t v)
    /// ```
    ///
    /// # Captura de variáveis
    ///
    /// ```text
    /// λa. (λx. λa. x a) (λz. a)
    /// ```
    /// <=> substituir `x` por `(λz. a)`
    /// ```text
    /// λa. (λa. (λz. a) a)
    /// ```
    ///
    /// Aqui ocorreu uma captura de variáveis, um erro, pois os dois `a` se
    /// referem a coisas diferentes, mas após a substituição, os dois não são
    /// distinguidos e aparentam ser ambos referências ao parâmetro mais
    /// interno.
    ///
    /// Solução mais básica? Trocar nome do parâmetro.
    /// ```text
    /// λa. (λa_. (λz. a) a_)
    /// ```
    ///
    /// No entanto, é preciso cuidar o seguinte:
    /// ```text
    /// λa_. λa. (λx. λa. x a a_) (λz. a) k
    /// ```
    /// <=> substituir `x` por `(λz. a)`
    /// ```text
    /// λa_. λa. (λa_. (λz. a) a_ a_) k
    /// ```
    ///
    /// Mas a resposta correta deve ser:
    /// ```text
    /// λa_. λa. (λa__. (λz. a) a__ a_) k
    /// ```
    ///
    /// Solução mais básica? Aumentar o nome da variável com `_` até não haver
    /// variáveis livres.
    pub fn replace(&mut self, target_var: &str, new_value: &Self) {
        /// Argumentos de uma substituição com variável a ser substituída e novo valor.
        enum Replacement<'this, 'var, 'new_value> {
            /// Substituição principal requisitada ao chamar o método [`Value::replace`].
            Main {
                /// Variável alvo da substituição (e.g. a ser substituída).
                target_var: &'var str,
                /// Novo valor substituindo a variável alvo.
                new_value: &'new_value Value,
                /// O conjunto de variáveis não-ligadas no novo valor.
                new_val_unbound_vars: HashSet<&'new_value str>,
            },
            /// Substituição intermediária para renomear o parâmetro de um lambda.
            Rename {
                /// Nome antigo do parâmetro.
                old_parameter: String,
                /// Novo nome do parâmetro.
                new_parameter: &'this str,
            },
        }

        impl<'this, 'var, 'new_value> Replacement<'this, 'var, 'new_value> {
            /// Testa se a dada variável é não-ligada dentro do "novo valor" desta substituição
            /// (i.e. o valor que vai substituir a variável alvo).
            fn is_unbound_var(&self, var_name: &str) -> bool {
                match self {
                    Replacement::Main { new_val_unbound_vars, .. } => {
                        new_val_unbound_vars.contains(var_name)
                    }
                    Replacement::Rename { new_parameter, .. } => {
                        *new_parameter == var_name
                    }
                }
            }

            /// Retorna o nome da variável alvo da substituição, isto é, a variável a ser substituída.
            fn target_var(&self) -> &str {
                match self {
                    Replacement::Main { target_var, .. } => target_var,
                    Replacement::Rename { old_parameter, .. } => old_parameter,
                }
            }

            /// Clona recursos e retorna o novo termo a ser usado na substituição.
            fn clone_new_value(&self) -> Value {
                match self {
                    Replacement::Main { new_value, .. } => (*new_value).clone(),
                    Replacement::Rename { new_parameter, .. } => {
                        Value::Variable((*new_parameter).to_owned())
                    }
                }
            }
        }

        /// Uma operação/passo para realizar a substituição requisitada.
        enum Operation<'this> {
            /// Performa todas as substituições do vetor de substituições dentro do tamanho máximo informado nesta operação.
            Replace(&'this mut Value, usize),
            /// Remove uma substituição a ser realizada do vetor de substituições, usando o índice informado.
            DropReplacement(usize),
        }

        // Argumentos de todas as substituições usadas pelas operações.
        let mut replacements = vec![Replacement::Main {
            target_var,
            new_value,
            new_val_unbound_vars: new_value.unbound_vars().collect(),
        }];
        // Operações: responsáveis por orquestrarem os passos a partir da
        // operação inicial, usando as substituições.
        let mut operation_stack = vec![Operation::Replace(self, 1)];

        while let Some(operation) = operation_stack.pop() {
            match operation {
                Operation::Replace(value, soft_size) => match value {
                    Value::Variable(variable) => {
                        if let Some(position) = replacements[..soft_size]
                            .iter()
                            // Último replacement que satisfaz a nossa condição.
                            .rposition(|replacement| {
                                variable == replacement.target_var()
                            })
                        {
                            // Troca o conteúdo de value.
                            *value = replacements[position].clone_new_value();
                            // Adiciona operação para tentar substituições que
                            // vierem antes dessa, usando como alvo o mesmo
                            // value.
                            operation_stack
                                .push(Operation::Replace(value, position));
                        }
                    }

                    Value::Application { function, argument } => {
                        operation_stack
                            .push(Operation::Replace(argument, soft_size));
                        operation_stack
                            .push(Operation::Replace(function, soft_size));
                    }

                    Value::Lambda { parameter, body } => {
                        // Vetor de substituições =
                        //  [
                        //      substituir x por (f a),
                        //      substituir y por (g (g b)),
                        //      substituir z por (c (h c)),
                        //      substituir w por (λf. f f),
                        //      substituir v por (e),
                        //  ]
                        //
                        // Expressão atual = λz. (x y z w v)
                        //
                        // Note que a substituição de z (índice 2) entra em
                        // conflito com o parâmetro λz, então a partir de z
                        // todas substituições devem ser bloqueadas, porque a
                        // substituição posterior é consequência da atual
                        // (delimitadas por "soft size").
                        //
                        // Portanto, somente as substituições x e y podem ser
                        // feitas (índices 0 e 1), e 2 é o novo "soft size".
                        let mut new_soft_size = replacements[..soft_size]
                            .iter()
                            // Posição do primeiro replacement que satisfizer a
                            // nossa condição.
                            .position(|replacement| {
                                parameter == replacement.target_var()
                            })
                            // Se não houver tal replacement, simplesmente use o
                            // "soft size" de antes.
                            .unwrap_or(soft_size);

                        // Função (closure) para testar se uma variável é livre
                        // em algum replacement válido aqui dentro.
                        let is_unbound_var = |variable: &str| {
                            replacements[..new_soft_size].iter().any(
                                |replacement| {
                                    replacement.is_unbound_var(variable)
                                },
                            )
                        };
                        let mut param_unbound = is_unbound_var(parameter);

                        if param_unbound {
                            let body_unbound: HashSet<_> =
                                body.unbound_vars().collect();
                            let mut renamed_var = format!("{}_", parameter);

                            // Enquanto a nova variável for livre no corpo
                            // do lambda ou em algum replacement:
                            while param_unbound {
                                param_unbound = is_unbound_var(&renamed_var)
                                    || body_unbound
                                        .contains(renamed_var.as_str());
                                if param_unbound {
                                    renamed_var.push('_');
                                }
                            }

                            // Substituí somente o atributo "parameter" de Value::Lambda
                            let old_parameter =
                                mem::replace(parameter, renamed_var);

                            // Adiciona um renomeamento de variáveis para o
                            // corpo do lambda.
                            //
                            // .insert() automaticamente abre espaço para
                            // inserir no índice desejado.
                            replacements.insert(
                                new_soft_size,
                                Replacement::Rename {
                                    old_parameter,
                                    new_parameter: parameter.as_str(),
                                },
                            );

                            // Destruirá o renomeamento logo após passar pelo
                            // corpo do lambda.
                            operation_stack.push(Operation::DropReplacement(
                                new_soft_size,
                            ));

                            // Conta a nova substituição (o renomeamento).
                            new_soft_size += 1;
                        }

                        operation_stack
                            .push(Operation::Replace(body, new_soft_size));
                    }
                },

                Operation::DropReplacement(index) => {
                    // .remove() automaticamente shifta elementos após o
                    // elemento
                    replacements.remove(index);
                }
            }
        }
    }

    /// Faz a redução de um único redex, mais externo, mais à esquerda. Retorna se tal redex foi encontrado.
    ///
    /// # Algoritmo Recursivo
    ///
    /// ```haskell
    /// reduceOne :: Value -> Maybe Value
    ///
    /// reduceOne (Variable s) = Nothing
    ///
    /// reduceOne (Application (Lambda p b) a) = Just (replace b p a)
    ///
    /// reduceOne (Application f a) = case reduceOne f of
    ///   Just f' -> Just (Application f' a)
    ///   Nothing -> case reduceOne a of
    ///     Just a' -> Just (Application f a')
    ///     Nothing -> Nothing
    ///
    /// reduceOne (Lambda p b) = case reduceOne b of
    ///   Just b' -> Just (Lambda p b')
    ///   Nothing -> Nothing
    ///
    ///
    /// reduceN :: Int -> Value -> Value
    /// reduceN 0 v = v
    /// reduceN n v = case reduceOne v of
    ///   Just v' -> reduceN (n - 1) v'
    ///   Nothing -> v
    ///
    ///
    /// reduceToNormal :: Value -> Value
    /// reduceToNormal v = case reduceOne v of
    ///   Just v' -> reduceToNormal v'
    ///   Nothing -> v
    /// ```
    pub fn reduce_one(&mut self) -> bool {
        let mut candidate_stack: Vec<&mut Value> = vec![self];
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
                        // NestedValues automaticamente convertidos para Values
                        // por conta do auto-deref.
                        candidate_stack.push(argument);
                        // Pela estratégia normal de avaliação, termos mais à
                        // esquerda são avaliados antes. A "função" em uma
                        // aplicação é quem está mais à esquerda (e mais afora),
                        // logo, precisa-se tentar avaliá-la antes
                        // (lembra que a pilha inverte).
                        candidate_stack.push(function);
                    }
                    Value::Lambda { parameter: _, body } => {
                        // Aqui também rola auto-deref.
                        candidate_stack.push(body);
                    }
                }
            }
        }

        redex_found
    }

    /// Cria um iterador sobre as variáveis não-ligadas neste termo. Variáveis podem aparecer mais de uma vez.
    ///
    /// # Algoritmo Recursivo
    ///
    /// ```haskell
    /// unboundVars :: Value -> [String]
    /// unboundVars v =
    ///   let unboundVarsWith (Variable s) bound =
    ///         if elem s bound
    ///           then []
    ///           else [s]
    ///
    ///       unboundVarsWith (Application f a) bound =
    ///         (unboundVarsWith f bound) ++ (unboundVarsWith a bound)
    ///
    ///       unboundVarsWith (Lambda p b) bound =
    ///         unboundVarsWith b (p : bound)
    ///
    ///   in unboundVarsWith v []
    /// ```
    pub fn unbound_vars(&self) -> UnboundVars {
        UnboundVars {
            operation_stack: vec![UnboundVarsOper::Visit(self)],
            bound_set: HashSet::new(),
        }
    }
}

impl PartialEq for Value {
    /// # Algoritmo recursivo
    ///
    /// ```haskell
    /// instance Eq Value where
    ///   (Variable s1) == (Variable s2) = s1 == s2
    ///   (Application f1 a1) == (Application f2 a2) = f1 == f2 && a1 == a2
    ///   (Lambda p1 b1) == (Lambda p2 b2) = p1 == p2 && b1 == b2
    ///   _ == _ = False
    /// ```
    fn eq(&self, other: &Self) -> bool {
        let mut equals = true;
        let mut compare_stack: Vec<(&Self, &Self)> = vec![(self, other)];

        while let Some((self_value, other_value)) =
            compare_stack.pop().filter(|_| equals)
        {
            match (self_value, other_value) {
                // Caso base
                (Value::Variable(self_var), Value::Variable(other_var)) => {
                    equals = self_var == other_var;
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
                    compare_stack.push((self_arg, other_arg));
                    compare_stack.push((self_func, other_func));
                }

                (
                    Value::Lambda { body: self_body, parameter: self_param },
                    Value::Lambda { body: other_body, parameter: other_param },
                ) => {
                    equals = self_param == other_param;
                    if equals {
                        compare_stack.push((self_body, other_body));
                    }
                }

                _ => equals = false,
            }
        }

        equals
    }
}

impl Clone for Value {
    /// # Algoritmo recursivo
    ///
    /// ```haskell
    /// -- Note that this useless in Haskell, it is only for mimicking the equivalent
    /// -- Rust cloning algorithm.
    /// cloneValue :: Value -> Value
    /// cloneValue (Variable s) = Variable s
    /// cloneValue (Application f a) = Application (cloneValue f) (cloneValue a)
    /// cloneValue (Lambda p b) = Lambda p (cloneValue b)
    /// ```
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

        let mut operation_stack: Vec<Operation> = vec![Operation::Clone(self)];
        let mut output_stack: Vec<Value> = Vec::new();

        while let Some(operation) = operation_stack.pop() {
            match operation {
                Operation::Clone(value) => match value {
                    // Caso base
                    Value::Variable(variable) => {
                        output_stack.push(Value::Variable(variable.clone()));
                    }

                    Value::Application { function, argument } => {
                        // .pop() retorna elementos na ordem inversa do .push()
                        operation_stack.push(Operation::MakeApplication);
                        operation_stack.push(Operation::Clone(argument));
                        operation_stack.push(Operation::Clone(function));
                    }
                    Value::Lambda { parameter, body } => {
                        // .pop() retorna elementos na ordem inversa do .push()
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
                            .push(UnboundVarsOper::Visit(argument));
                        self.operation_stack
                            .push(UnboundVarsOper::Visit(function));
                    }
                    Value::Lambda { parameter, body } => {
                        // Múltiplos lambdas, aninhados, com mesmo parâmetro
                        // tentam inserir o parâmetro mais de uma vez. Mas ele
                        // só pode ser efetivamente inserido uma única vez,
                        // assim como só pode ser removido uma vez. Por isso,
                        // vamos deixar somente para o lambda mais de fora
                        // remover o parâmetro.
                        let is_new_element = self.bound_set.insert(parameter);
                        if is_new_element {
                            // .pop() retorna elementos na ordem inversa do .push()
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
                // Caso base
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

/// Mapeamento de nomes de parâmetros para índices.
///
/// Serve para computar índices de De Bruijn de um termo.
///
/// Ìndices de De Bruijn:
///
/// ```text
///  4   3   2   1  4 1
///  |   |   |   |  | |
///  V   V   V   V  V V
/// \a. \x. \c. \x. a x
///
/// ==>
///
/// \. \. \. \. 4 1
/// ```
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
        // Como é armazenado:
        //
        //  0   1   2   3
        //  |   |   |   |
        //  V   V   V   V
        // \a. \x. \c. \x. x
        //
        //
        // Ìndices de De Bruijn:
        //
        //  4   3   2   1  4 1
        //  |   |   |   |  | |
        //  V   V   V   V  V V
        // \a. \x. \c. \x. a x
        //
        // Por isso a correção de índice com o .map()
        self.param_map.get(param).copied().map(|index| self.param_count - index)
    }
}
