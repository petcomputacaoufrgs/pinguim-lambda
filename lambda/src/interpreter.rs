//! Esse módulo exporta o interpretador de cálculo lambda com contagem de passos.

#[cfg(test)]
mod test;

use crate::value::Value;

/// Reduz o termo até a sua forma normal, se existir. Se não existir, entra em loop infinito.
pub fn run_once(input: Value) -> Value {
    let mut interpreter = Interpreter::new(input);
    interpreter.run_all();
    interpreter.finish()
}

/// Dados do interpretador, tal como passos dados, entrada original e termo atual.
#[derive(Debug, Clone)]
pub struct Interpreter {
    /// Passos dados desde o início da interpretação.
    steps: u64,
    /// Entrada original.
    input: Value,
    /// Termo atual.
    current: Value,
}

impl Interpreter {
    /// Cria um interpretador a partir do termo de entrada, com passos zerados.
    pub fn new(input: Value) -> Self {
        Self { current: input.clone(), input, steps: 0 }
    }

    /// Reseta o status do interpretador para o início.
    pub fn reset(&mut self) {
        self.steps = 0;
        self.current = self.input.clone();
    }

    /// Altera a entrada original, resetando o status do interpretador.
    pub fn set_input(&mut self, input: Value) {
        self.input = input;
        self.reset();
    }

    /// Roda um passo da redução.
    /// Retorna `true` se houve redução.
    pub fn run_step(&mut self) -> bool {
        if self.current.reduce_one() {
            self.steps += 1;
            true
        } else {
            false
        }
    }

    /// Roda um determinado número de passos de redução.
    /// Retorna `true` se todos os passos indicados foram executados.
    /// Retorna `false` se todas as reduções possíveis foram feitas antes de chegar no limite de passos.
    pub fn run_steps(&mut self, max_steps: u32) -> bool {
        for _ in 0..max_steps {
            if !self.run_step() {
                return false;
            }
        }
        true
    }

    /// Tenta realizar todas as reduções possíveis, até a forma normal.
    /// Se não houver forma normal, entra em loop infinito.
    pub fn run_all(&mut self) {
        while self.run_step() {}
    }

    /// Retorna quantos passos foram dados.
    pub fn steps(&self) -> u64 {
        self.steps
    }

    /// Retorna uma referência para a entrada original.
    pub fn input(&self) -> &Value {
        &self.input
    }

    /// Retorna uma referência para a saída atualmente computada.
    pub fn output(&self) -> &Value {
        &self.current
    }

    /// Consome o interpretador e retorna a saída final.
    pub fn finish(self) -> Value {
        self.current
    }
}
